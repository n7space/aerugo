//! Mutex for x86.
use core::cell::UnsafeCell;

/// Mutex for x86
///
/// # Generic Parameters
/// * `T` - Type of the stored value.
#[repr(transparent)]
pub struct Mutex<T: ?Sized>(UnsafeCell<T>);

/// x86 works only on one thread.
unsafe impl<T: Send + ?Sized> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates new mutex with given value
    ///
    /// # Parameters
    /// * `value` - Value to initialize the cell with.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Mutex(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Gives access to the value by locking the mutex
    ///
    /// # Parameters
    /// * `f` - Lambda to execute.
    ///
    /// # Return
    /// Result of the executed lambda.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { f(self.as_mut_ref()) }
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
