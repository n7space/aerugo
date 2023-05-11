//! TODO

use core::cell::UnsafeCell;
use core::fmt::{Debug, Error, Formatter};

/// TODO
#[repr(transparent)]
pub(crate) struct InternalCell<T: ?Sized>(UnsafeCell<T>);

/// TODO
unsafe impl<T: ?Sized> Sync for InternalCell<T> where T: Send {}

impl<T> InternalCell<T> {
    /// TODO
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        InternalCell(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> InternalCell<T> {
    /// TODO
    #[inline(always)]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.0.get()
    }

    /// TODO
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
