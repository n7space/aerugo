//! Mutex based on the critical section.
//!
//! This mutex is used for the safe access to the data that have to be declared as static. Access to
//! the internal value can be only done by performing a lock on the mutex which enables critical
//! section for the duration.

use core::cell::UnsafeCell;

/// Mutex based on the critical section.
///
/// # Generic Parameters
/// * `T` - Type of the stored value.
#[repr(transparent)]
pub struct Mutex<T: ?Sized>(UnsafeCell<T>);

/// Mutex is `Sync` because `aerugo` is a single-threaded system and critical section prevents any access
/// to the data from interrupts. Value cannot be borrowed outside of the critical section.
unsafe impl<T: Send + ?Sized> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates new mutex with given value
    ///
    /// # Parameters
    /// * `value` - Value to initialize the mutex with.
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
    /// # Parameters
    /// * `f` - Lambda to execute.
    ///
    /// # Return
    /// Result of the executed lambda.
    #[inline(always)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { critical_section::with(|_| f(self.as_mut_ref())) }
    }

    /// Returns a mutable reference to the stored value.
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
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
