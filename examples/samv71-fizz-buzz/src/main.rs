#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    log, logln, Aerugo, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
    EventId, EventStorage, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};

use rt::entry;

struct ProducerContext {
    acc: u8,
    elem_queue_handle: MessageQueueHandle<u8, 10>,
    generate_numbers_condition_handle: BooleanConditionHandle,
}

fn producer(_: (), context: &mut ProducerContext, _: &dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);

    context
        .elem_queue_handle
        .send_data(context.acc)
        .expect("Failed to send data to elem Queue");

    if context.acc == u8::MAX {
        context.generate_numbers_condition_handle.set_value(false);
    }
}

#[derive(Default)]
struct DistributorContext {}

fn distributor(val: u8, _: &mut DistributorContext, api: &'static dyn RuntimeApi) {
    match (val % 3, val % 5) {
        (0, 0) => api
            .emit_event(FizzBuzzEvents::FizzBuzz.into())
            .expect("Failed to emit FizzBuzz"),
        (0, _) => api
            .emit_event(FizzBuzzEvents::Fizz.into())
            .expect("Failed to emit Fizz"),
        (_, 0) => api
            .emit_event(FizzBuzzEvents::Buzz.into())
            .expect("Failed to emit Buzz"),
        _ => log!("{}\n", val),
    }
}

#[derive(Default)]
struct FizzContext {}

fn fizz(val: EventId, _: &mut FizzContext, _: &'static dyn RuntimeApi) {
    log!("Fizz");

    if let FizzBuzzEvents::Fizz = val.into() {
        log!("\n");
    }
}

#[derive(Default)]
struct BuzzContext {}

fn buzz(_: EventId, _: &mut BuzzContext, _: &'static dyn RuntimeApi) {
    log!("Buzz\n");
}

#[derive(Default)]
struct DoneContext {}

fn done(_: bool, _: &mut DoneContext, _: &'static dyn RuntimeApi) {
    log!("Done!\n");
}

enum FizzBuzzEvents {
    Fizz,
    Buzz,
    FizzBuzz,
}

impl From<FizzBuzzEvents> for EventId {
    fn from(value: FizzBuzzEvents) -> Self {
        match value {
            FizzBuzzEvents::Fizz => 3,
            FizzBuzzEvents::Buzz => 5,
            FizzBuzzEvents::FizzBuzz => 15,
        }
    }
}

impl From<EventId> for FizzBuzzEvents {
    fn from(value: EventId) -> Self {
        match value {
            3 => FizzBuzzEvents::Fizz,
            5 => FizzBuzzEvents::Buzz,
            15 => FizzBuzzEvents::FizzBuzz,
            _ => unreachable!(),
        }
    }
}

static PRODUCER_STORAGE: TaskletStorage<(), ProducerContext, 1> = TaskletStorage::new();
static DISTRIBUTOR_STORAGE: TaskletStorage<u8, DistributorContext, 0> = TaskletStorage::new();
static FIZZ_STORAGE: TaskletStorage<EventId, FizzContext, 0> = TaskletStorage::new();
static BUZZ_STORAGE: TaskletStorage<EventId, BuzzContext, 0> = TaskletStorage::new();
static DONE_STORAGE: TaskletStorage<bool, DoneContext, 0> = TaskletStorage::new();

static ELEM_QUEUE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

static FIZZ_EVENT_STORAGE: EventStorage = EventStorage::new();
static BUZZ_EVENT_STORAGE: EventStorage = EventStorage::new();
static FIZZ_BUZZ_EVENT_STORAGE: EventStorage = EventStorage::new();

static GENERATE_NUMBERS_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    aerugo
        .create_message_queue(&ELEM_QUEUE)
        .expect("Unable to create ElemQueue");

    aerugo
        .create_event(FizzBuzzEvents::Fizz.into(), &FIZZ_EVENT_STORAGE)
        .expect("Unable to create Fizz event");
    aerugo
        .create_event(FizzBuzzEvents::Buzz.into(), &BUZZ_EVENT_STORAGE)
        .expect("Unable to create Buzz event");
    aerugo
        .create_event(FizzBuzzEvents::FizzBuzz.into(), &FIZZ_BUZZ_EVENT_STORAGE)
        .expect("Unable to create FizzBuzz event");

    aerugo
        .create_boolean_condition(&GENERATE_NUMBERS_CONDITION_STORAGE, true)
        .expect("Unable to create GenerateNumbersCondition");

    let elem_queue_handle = ELEM_QUEUE
        .create_handle()
        .expect("Unable to create ElemQueue");

    let generate_numbers_condition_handle = GENERATE_NUMBERS_CONDITION_STORAGE
        .create_handle()
        .expect("Unable to create GenerateNumbersCondition handle");

    let producer_config = TaskletConfig {
        name: "Producer",
        ..Default::default()
    };
    let producer_context = ProducerContext {
        acc: 0,
        elem_queue_handle,
        generate_numbers_condition_handle,
    };

    aerugo
        .create_tasklet_with_context(
            producer_config,
            producer,
            producer_context,
            &PRODUCER_STORAGE,
        )
        .expect("Unable to create Producer");

    let distributor_config = TaskletConfig {
        name: "Distributor",
        priority: 1,
    };

    aerugo
        .create_tasklet(distributor_config, distributor, &DISTRIBUTOR_STORAGE)
        .expect("Unable to create Distributor");

    let fizz_config = TaskletConfig {
        name: "Fizz",
        priority: 3,
    };

    aerugo
        .create_tasklet(fizz_config, fizz, &FIZZ_STORAGE)
        .expect("Unable to create Fizz");

    let buzz_config = TaskletConfig {
        name: "Buzz",
        priority: 2,
    };

    aerugo
        .create_tasklet(buzz_config, buzz, &BUZZ_STORAGE)
        .expect("Unable to create Buzz");

    let done_config = TaskletConfig {
        name: "Done",
        ..Default::default()
    };

    aerugo
        .create_tasklet(done_config, done, &DONE_STORAGE)
        .expect("Unable to create Done");

    let producer_condition_set = BooleanConditionSet::from(generate_numbers_condition_handle);

    let producer_handle = PRODUCER_STORAGE
        .create_handle()
        .expect("Unable to create handle to Producer");
    aerugo
        .set_tasklet_conditions(&producer_handle, producer_condition_set)
        .expect("Unable to set Producer condition set");
    aerugo
        .subscribe_tasklet_to_cyclic(&producer_handle, None)
        .expect("Unable to set cyclic on Producer");

    let distributor_handle = DISTRIBUTOR_STORAGE
        .create_handle()
        .expect("Unable to create handle to Distributor");
    aerugo
        .subscribe_tasklet_to_queue(&distributor_handle, &elem_queue_handle)
        .expect("Unable to subscribe Distributor to ElemQueue");

    let fizz_handle = FIZZ_STORAGE
        .create_handle()
        .expect("Unable to create handle to Fizz");
    let fizz_events = [FizzBuzzEvents::Fizz.into(), FizzBuzzEvents::FizzBuzz.into()];
    aerugo
        .subscribe_tasklet_to_events(&fizz_handle, fizz_events)
        .expect("Unable to subscribe Fizz to events");

    let buzz_handle = BUZZ_STORAGE
        .create_handle()
        .expect("Unable to create handle to Buzz");
    let buzz_events = [FizzBuzzEvents::Buzz.into(), FizzBuzzEvents::FizzBuzz.into()];
    aerugo
        .subscribe_tasklet_to_events(&buzz_handle, buzz_events)
        .expect("Unable to subscribe Buzz to events");

    let done_handle = DONE_STORAGE
        .create_handle()
        .expect("Unable to create handle to Done");
    aerugo
        .subscribe_tasklet_to_condition(&done_handle, &generate_numbers_condition_handle)
        .expect("Unable to subscribe Done to GenerateNumbersCondition");

    aerugo.start();
}
