//! Unsafe cell that implements Sync and Send
//!
//! This cell should be only used in the internal parts of the system for elements that are
//! static and are initialized (and by this - modified) only once - before the system is started.
//!
//! SAFETY: Borrows has to be managed by hand, there are no mechanism checking if mutable and
//! immutable reference exists at the same time.

use core::cell::UnsafeCell;
use core::fmt::{Debug, Error, Formatter};

/// Unsafe sync cell.
#[repr(transparent)]
pub(crate) struct InternalCell<T: ?Sized>(UnsafeCell<T>);

/// Safety of the InternalCell has to be managed by hand.
unsafe impl<T: Send + ?Sized> Sync for InternalCell<T> {}

impl<T> InternalCell<T> {
    /// Creates new cell.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        InternalCell(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> InternalCell<T> {
    /// Borrows an immutable reference to the value.
    ///
    /// SAFETY: There are no borrow checking mechanism, safety has to be managed by hand.
    ///
    /// Returns reference to the value.
    #[inline(always)]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.0.get()
    }

    /// Borrows a mutable reference to the value.
    ///
    /// SAFETY: There are no borrow checking mechanism, safety has to be managed by hand.
    ///
    /// Returns reference to the value.
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn as_mut_ref(&self) -> &mut T {
        &mut *self.0.get()
    }
}

impl<T: Default> Default for InternalCell<T> {
    fn default() -> InternalCell<T> {
        InternalCell::new(Default::default())
    }
}

impl<T> From<T> for InternalCell<T> {
    fn from(t: T) -> InternalCell<T> {
        InternalCell::new(t)
    }
}

impl<T: Debug> Debug for InternalCell<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe { self.as_ref().fmt(fmt) }
    }
}
