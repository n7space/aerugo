//! This module stores list used for internals of the system.

use core::cell::UnsafeCell;
use core::iter::IntoIterator;
use core::ops::Deref;
use core::slice::Iter;

/// Type of the stored list.
type ListType<T, const N: usize> = heapless::Vec<T, N>;

/// Internal list.
///
/// This is used when some list has to be created and filled at the system initialization. Adding
/// elements is safe only before system starts. After startup elements can be safely accessed using
/// via const reference.
pub(crate) struct InternalList<T, const N: usize> {
    /// Stored list
    list: UnsafeCell<ListType<T, N>>,
}

impl<T, const N: usize> InternalList<T, N> {
    /// Creates new list
    pub const fn new() -> Self {
        Self {
            list: UnsafeCell::new(ListType::new()),
        }
    }

    /// Adds element to the list.
    ///
    /// # Safety
    /// This is unsafe because it modified the stored list. Stored cell is not leaked, so this is
    /// considered safe before system initialization, as no other reference shall exist.
    pub unsafe fn add(&self, elem: T) -> Result<(), T> {
        (*self.list.get()).push(elem)
    }
}

impl<T, const N: usize> Deref for InternalList<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe, because reference of the stored cell is not leaked outside while
        // list is modified, so no other mutable reference can exist at the same time.
        unsafe { (*self.list.get()).as_slice() }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a InternalList<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: This is safe, because reference of the stored cell is not leaked outside while
        // list is modified, so no other mutable reference can exist at the same time.
        unsafe { (&*self.list.get()).into_iter() }
    }
}
