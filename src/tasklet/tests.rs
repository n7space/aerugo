use super::*;

use crate::boolean_condition::BooleanConditionSetType;

struct MockDataProvider {
    data_ready: bool,
}

impl MockDataProvider {
    const fn new() -> Self {
        MockDataProvider { data_ready: false }
    }

    fn set_data_ready(&mut self, data_ready: bool) {
        self.data_ready = data_ready
    }
}

impl DataProvider<()> for MockDataProvider {
    fn data_ready(&self) -> bool {
        self.data_ready
    }

    fn get_data(&self) -> Option<()> {
        if self.data_ready {
            Some(())
        } else {
            None
        }
    }
}

struct MockConditionSet<const N: usize> {
    pub storage: OnceCell<BooleanConditionSet<N>>,
}

unsafe impl<const N: usize> Sync for MockConditionSet<N> {}

impl<const N: usize> MockConditionSet<N> {
    const fn new() -> Self {
        MockConditionSet {
            storage: OnceCell::new(),
        }
    }
}

/// @SRS{ROS-FUN-RTOS-50}
/// @SRS{ROS-FUN-RTOS-60}
/// @SRS{ROS-FUN-RTOS-70}
/// @SRS{ROS-FUN-RTOS-80}
#[cfg_attr(not(doc), test)]
fn req_tasklet_execution_state() {
    static mut MOCK_DATA_PROVIDER: MockDataProvider = MockDataProvider::new();
    static MOCK_CONDITION_SET: MockConditionSet<0> = MockConditionSet::new();
    let _ = MOCK_CONDITION_SET
        .storage
        .set(BooleanConditionSet::new(BooleanConditionSetType::And));

    static mut TASKLET_CONTEXT: () = ();
    let tasklet_config = TaskletConfig {
        name: "TestTasklet",
        ..Default::default()
    };
    let tasklet: Tasklet<(), (), 0> = Tasklet::new(
        tasklet_config,
        |_, _| {},
        unsafe { &mut TASKLET_CONTEXT },
        &MOCK_CONDITION_SET.storage,
    );

    let subscribe_result = unsafe { tasklet.subscribe(&MOCK_DATA_PROVIDER) };
    assert!(subscribe_result.is_ok());

    // Execution status is managed by the executor.
    assert_eq!(tasklet.get_status(), TaskletStatus::Sleeping);

    assert!(!tasklet.is_ready());
    unsafe {
        MOCK_DATA_PROVIDER.set_data_ready(true);
    };
    assert!(tasklet.is_ready());
}

#[test]
fn const_size() {
    type TaskletStub = Tasklet<(), (), 0>;
    let stub_size = core::mem::size_of::<TaskletStub>();

    struct NoCtx;
    type TaskletNoCtx = Tasklet<u8, NoCtx, 1>;
    let noctx_size = core::mem::size_of::<TaskletNoCtx>();

    struct SmallCtx {
        _a: u16,
    }
    type TaskletSmallCtx = Tasklet<u16, SmallCtx, 2>;
    let smallctx_size = core::mem::size_of::<TaskletSmallCtx>();

    struct BigCtx {
        _a: u64,
        _b: f64,
        _c: u16,
    }
    type TaskletBigCtx = Tasklet<u32, BigCtx, 5>;
    let bigctx_size = core::mem::size_of::<TaskletBigCtx>();

    assert_eq!(noctx_size, stub_size);
    assert_eq!(smallctx_size, stub_size);
    assert_eq!(bigctx_size, stub_size);
}
