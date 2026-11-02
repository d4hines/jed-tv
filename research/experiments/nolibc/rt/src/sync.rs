//! A futex mutex (0 = unlocked, 1 = locked, 2 = locked with waiters) and a
//! wrapper for global state guarded by it.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};

const FUTEX_WAIT_PRIVATE: usize = 128;
const FUTEX_WAKE_PRIVATE: usize = 129;

pub fn futex_wait(word: &AtomicU32, val: u32, private: bool) {
    let op = if private { FUTEX_WAIT_PRIVATE } else { 0 };
    unsafe { sc!(libc::SYS_futex, word as *const AtomicU32, op, val, 0usize) };
}

pub fn futex_wake(word: &AtomicU32, n: u32, private: bool) {
    let op = if private { FUTEX_WAKE_PRIVATE } else { 1 };
    unsafe { sc!(libc::SYS_futex, word as *const AtomicU32, op, n) };
}

pub struct Lock(AtomicU32);

impl Lock {
    pub const fn new() -> Self {
        Lock(AtomicU32::new(0))
    }

    pub fn lock(&self) {
        if self.0.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            return;
        }
        while self.0.swap(2, Ordering::Acquire) != 0 {
            futex_wait(&self.0, 2, true);
        }
    }

    pub fn unlock(&self) {
        if self.0.swap(0, Ordering::Release) == 2 {
            futex_wake(&self.0, 1, true);
        }
    }
}

/// Global state of type T, only touched while holding its lock.
pub struct Locked<T> {
    lock: Lock,
    val: UnsafeCell<T>,
}

unsafe impl<T> Sync for Locked<T> {}

impl<T> Locked<T> {
    pub const fn new(v: T) -> Self {
        Locked { lock: Lock::new(), val: UnsafeCell::new(v) }
    }

    pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.lock.lock();
        let r = f(unsafe { &mut *self.val.get() });
        self.lock.unlock();
        r
    }

    /// For fork(): hold the lock across the clone so the child gets a
    /// consistent copy, then release it in both processes.
    pub fn raw_lock(&self) -> &Lock {
        &self.lock
    }
}

/// Global written once at startup (single-threaded), read-only afterwards.
pub struct Racy<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Racy<T> {}

impl<T: Copy> Racy<T> {
    pub const fn new(v: T) -> Self {
        Racy(UnsafeCell::new(v))
    }
    pub fn get(&self) -> T {
        unsafe { *self.0.get() }
    }
    /// # Safety
    /// Only during single-threaded startup.
    pub unsafe fn set(&self, v: T) {
        *self.0.get() = v;
    }
}
