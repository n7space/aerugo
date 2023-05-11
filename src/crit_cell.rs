//! TODO

use core::cell::UnsafeCell;
use core::fmt::{Debug, Error, Formatter};

use cortex_m::interrupt;

/// TODO
#[repr(transparent)]
pub struct CritCell<T: ?Sized>(UnsafeCell<T>);

/// TODO
unsafe impl<T: ?Sized> Sync for CritCell<T> where T: Send {}

impl<T> CritCell<T> {
    /// TODO
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        CritCell(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> CritCell<T> {
    /// TODO
    #[inline(always)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { interrupt::free(|_| f(self.as_mut_ref())) }
    }

    /// TODO
    #[inline(always)]
    unsafe fn as_ref(&self) -> &T {
        &*self.0.get()
    }

    /// TODO
    #[inline(always)]
    unsafe fn as_mut_ref(&self) -> &mut T {
        &mut *self.0.get()
    }
}

impl<T: Default> Default for CritCell<T> {
    fn default() -> CritCell<T> {
        CritCell::new(Default::default())
    }
}

impl<T> From<T> for CritCell<T> {
    fn from(t: T) -> CritCell<T> {
        CritCell::new(t)
    }
}

impl<T: Debug> Debug for CritCell<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe { self.as_ref().fmt(fmt) }
    }
}
