#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as runtime;
extern crate panic_rtt_target;
extern crate rtt_target as rtt;

use aerugo::{
    time::MillisDurationU32, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
    EventId, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage, AERUGO,
};

use rtt::{rprint, rprintln};
use runtime::entry;

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
        _ => rprint!("{}\n", val),
    }
}

#[derive(Default)]
struct FizzContext {}

fn fizz(val: EventId, _: &mut FizzContext, _: &'static dyn RuntimeApi) {
    rprint!("Fizz");

    if let FizzBuzzEvents::Fizz = val.into() {
        rprint!("\n");
    }
}

#[derive(Default)]
struct BuzzContext {}

fn buzz(_: EventId, _: &mut BuzzContext, _: &'static dyn RuntimeApi) {
    rprint!("Buzz\n");
}

#[derive(Default)]
struct DoneContext {}

fn done(_: bool, _: &mut DoneContext, _: &'static dyn RuntimeApi) {
    rprint!("Done!\n");
}

static PRODUCER_STORAGE: TaskletStorage<(), ProducerContext, 1> = TaskletStorage::new();
static DISTRIBUTOR_STORAGE: TaskletStorage<u8, DistributorContext, 0> = TaskletStorage::new();
static FIZZ_STORAGE: TaskletStorage<EventId, FizzContext, 0> = TaskletStorage::new();
static BUZZ_STORAGE: TaskletStorage<EventId, BuzzContext, 0> = TaskletStorage::new();
static DONE_STORAGE: TaskletStorage<bool, DoneContext, 0> = TaskletStorage::new();

static ELEM_QUEUE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

static GENERATE_NUMBERS_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

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

#[entry]
fn main() -> ! {
    rtt::rtt_init_print!();

    rprintln!("Hello, world! Initializing Aerugo...");

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
    });

    AERUGO
        .create_message_queue(&ELEM_QUEUE)
        .expect("Unable to create ElemQueue");

    AERUGO
        .create_event(FizzBuzzEvents::Fizz.into())
        .expect("Unable to create Fizz event");
    AERUGO
        .create_event(FizzBuzzEvents::Buzz.into())
        .expect("Unable to create Buzz event");
    AERUGO
        .create_event(FizzBuzzEvents::FizzBuzz.into())
        .expect("Unable to create FizzBuzz event");

    AERUGO
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

    AERUGO
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

    AERUGO
        .create_tasklet(distributor_config, distributor, &DISTRIBUTOR_STORAGE)
        .expect("Unable to create Distributor");

    let fizz_config = TaskletConfig {
        name: "Fizz",
        priority: 3,
    };

    AERUGO
        .create_tasklet(fizz_config, fizz, &FIZZ_STORAGE)
        .expect("Unable to create Fizz");

    let buzz_config = TaskletConfig {
        name: "Buzz",
        priority: 2,
    };

    AERUGO
        .create_tasklet(buzz_config, buzz, &BUZZ_STORAGE)
        .expect("Unable to create Buzz");

    let done_config = TaskletConfig {
        name: "Done",
        ..Default::default()
    };

    AERUGO
        .create_tasklet(done_config, done, &DONE_STORAGE)
        .expect("Unable to create Done");

    let producer_condition_set = BooleanConditionSet::from(generate_numbers_condition_handle);

    let producer_handle = PRODUCER_STORAGE
        .create_handle()
        .expect("Unable to create handle to Producer");
    AERUGO
        .set_tasklet_conditions(&producer_handle, producer_condition_set)
        .expect("Unable to set Producer condition set");
    AERUGO
        .subscribe_tasklet_to_cyclic(&producer_handle, None)
        .expect("Unable to set cyclic on Producer");

    let distributor_handle = DISTRIBUTOR_STORAGE
        .create_handle()
        .expect("Unable to create handle to Distributor");
    AERUGO
        .subscribe_tasklet_to_queue(&distributor_handle, &elem_queue_handle)
        .expect("Unable to subscribe Distributor to ElemQueue");

    let fizz_handle = FIZZ_STORAGE
        .create_handle()
        .expect("Unable to create handle to Fizz");
    AERUGO
        .subscribe_tasklet_to_events(&fizz_handle)
        .expect("Unable to subscribe Fizz to events")
        .enable(FizzBuzzEvents::Fizz.into())
        .expect("Unable to enable Fizz event for Fizz")
        .enable(FizzBuzzEvents::FizzBuzz.into())
        .expect("Unable to enable FizzBuzz event for Fizz");

    let buzz_handle = BUZZ_STORAGE
        .create_handle()
        .expect("Unable to create handle to Buzz");
    AERUGO
        .subscribe_tasklet_to_events(&buzz_handle)
        .expect("Unable to subscribe Buzz to events")
        .enable(FizzBuzzEvents::Buzz.into())
        .expect("Unable to enable Buzz event for Buzz")
        .enable(FizzBuzzEvents::FizzBuzz.into())
        .expect("Unable to enable FizzBuzz event for Buzz");

    let done_handle = DONE_STORAGE
        .create_handle()
        .expect("Unable to create handle to Done");
    AERUGO
        .subscribe_tasklet_to_condition(&done_handle, &generate_numbers_condition_handle)
        .expect("Unable to subscribe Done to GenerateNumbersCondition");

    AERUGO.start();
}
