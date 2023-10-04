//! Boolean condition.

mod boolean_condition_handle;
mod boolean_condition_set;
mod boolean_condition_storage;

pub use self::boolean_condition_handle::BooleanConditionHandle;
pub use self::boolean_condition_set::BooleanConditionSet;
pub use self::boolean_condition_set::BooleanConditionSetType;
pub use self::boolean_condition_storage::BooleanConditionStorage;

use crate::aerugo::Aerugo;
use crate::data_provider::DataProvider;
use crate::error::SystemError;
use crate::internal_list::InternalList;
use crate::mutex::Mutex;
use crate::tasklet::TaskletPtr;

/// List of tasklets registered to a condition
type TaskletList = InternalList<TaskletPtr, { Aerugo::TASKLET_COUNT }>;

/// Boolean condition.
#[repr(C)]
pub(crate) struct BooleanCondition {
    /// Condition value.
    value: Mutex<bool>,
    /// Tasklets registered to this queue.
    registered_tasklets: TaskletList,
}

impl BooleanCondition {
    /// Creates new `BooleanCondition`
    pub(crate) fn new(value: bool) -> Self {
        BooleanCondition {
            value: Mutex::new(value),
            registered_tasklets: TaskletList::new(),
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
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of registered tasklets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), SystemError> {
        match self.registered_tasklets.add(tasklet) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::TaskletListFull),
        }
    }

    /// Wakes tasklets registered to this condition.
    fn wake_tasklets(&self) {
        for t in &self.registered_tasklets {
            Aerugo::wake_tasklet(t);
        }
    }
}

impl DataProvider<bool> for BooleanCondition {
    /// Returns state of the condition.
    ///
    /// # Return
    /// Always returns `Some(bool)`.
    fn get_data(&self) -> Option<bool> {
        self.value.lock(|v| Some(*v))
    }

    /// Returns false, as there is no waiting data in the condition.
    ///
    /// Condition has it's state that can be accessed, but it is not considered 'waiting' for the
    /// scheduling purposes.
    fn data_waiting(&self) -> bool {
        false
    }
}

unsafe impl Sync for BooleanCondition {}
