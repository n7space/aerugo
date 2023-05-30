//! Mutex based on the critical section.
//!
//! This mutex is used for the safe access to the data that have to be declared as static. Access to
//! the internal value can be only done by performing a lock on the mutex which enables critical
//! section for the duration.

use core::cell::UnsafeCell;

use cortex_m::interrupt;

/// Mutex based on the critical section.
#[repr(transparent)]
pub struct Mutex<T: ?Sized>(UnsafeCell<T>);

/// Mutex is safe to share between threads because critical section prevents any access to the
/// data from other threads or interrupts. Value cannot be borrowed outside of the
/// critical section.
unsafe impl<T: Send + ?Sized> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates new mutex with given value
    ///
    /// * `value` - Value to initialize the mutex with.
    ///
    /// Returns new mutex.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Mutex(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Gives access to the value in critical section.
    ///
    /// This is the only access to the value. Given lambda is passed a mutable reference to the
    /// value and executed in critical section. This ensures that the value won't be borrowed more
    /// than once at the given time.
    ///
    /// * `f` - Lambda to execute.
    ///
    /// Returns the result of the executed lambda.
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { interrupt::free(|_| f(self.as_mut_ref())) }
    }

    #[inline(always)]
    unsafe fn as_mut_ref(&self) -> &mut T {
        &mut *self.0.get()
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(t: T) -> Mutex<T> {
        Mutex::new(t)
    }
}
