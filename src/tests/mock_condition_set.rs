use crate::boolean_condition::BooleanConditionSet;
use core::cell::OnceCell;

pub(crate) struct MockConditionSet<const N: usize> {
    pub storage: OnceCell<BooleanConditionSet<N>>,
}

unsafe impl<const N: usize> Sync for MockConditionSet<N> {}

impl<const N: usize> MockConditionSet<N> {
    pub(crate) const fn new() -> Self {
        MockConditionSet {
            storage: OnceCell::new(),
        }
    }
}
