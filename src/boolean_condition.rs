//! Boolean condition.

mod boolean_condition_handle;
mod boolean_condition_set;
mod boolean_condition_storage;

pub use self::boolean_condition_handle::BooleanConditionHandle;
pub use self::boolean_condition_set::BooleanConditionSet;
pub use self::boolean_condition_set::BooleanConditionSetType;
pub use self::boolean_condition_storage::BooleanConditionStorage;

use heapless::Vec;

use crate::aerugo::{Aerugo, AERUGO};
use crate::api::{InitError, SystemApi};
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::internal_cell::InternalCell;
use crate::tasklet::TaskletPtr;

/// List of tasklets registered to a condition
type TaskletList = Vec<TaskletPtr, { Aerugo::TASKLET_COUNT }>;

/// Boolean condition.
#[repr(C)]
pub(crate) struct BooleanCondition {
    /// Condition value.
    value: Mutex<bool>,
    /// Tasklets registered to this queue.
    registered_tasklets: InternalCell<TaskletList>,
}

impl BooleanCondition {
    /// Creates new `BooleanCondition`
    pub(crate) fn new(value: bool) -> Self {
        BooleanCondition {
            value: Mutex::new(value),
            registered_tasklets: TaskletList::new().into(),
        }
    }

    /// Gets value of the condition.
    pub fn get_value(&self) -> bool {
        self.value.lock(|v| *v)
    }

    /// Sets value of the condition.
    pub fn set_value(&self, value: bool) {
        let value_changed = self.value.lock(|v| {
            if *v != value {
                *v = value;
                true
            } else {
                false
            }
        });

        if value_changed {
            self.wake_tasklets();
        }
    }

    /// Registers tasklet to this condition
    ///
    /// # Parameter
    /// * `tasklet` - Tasklet to register
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of registered tasklets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError> {
        match self.registered_tasklets.as_mut_ref().push(tasklet) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::TaskletListFull),
        }
    }

    /// Wakes tasklets registered to this condition.
    fn wake_tasklets(&self) {
        // SAFETY: This is safe, because no mutable references should be able to exist at the same time.
        for t in unsafe { self.registered_tasklets.as_ref() } {
            AERUGO.wake_tasklet(t);
        }
    }
}

impl DataProvider<bool> for BooleanCondition {
    fn data_ready(&self) -> bool {
        true
    }

    fn get_data(&self) -> Option<bool> {
        self.value.lock(|v| Some(*v))
    }
}
