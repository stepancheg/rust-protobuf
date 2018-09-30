//! Lazily initialized data.
//! Used in generated code.

use std::sync;
use std::cell::UnsafeCell;

/// Lasily initialized data.
// Fields are public until `const` functions available in stable.
pub struct Lazy<T> {
    #[doc(hidden)]
    pub lock: sync::Once,
    #[doc(hidden)]
    pub ptr: UnsafeCell<*const T>,
}

unsafe impl<T> Sync for Lazy<T> {}

impl<T> Lazy<T> {
    /// Get lazy field value, initialize it with given function if not yet.
    pub fn get<F>(&'static self, init: F) -> &'static T
    where
        F : FnOnce() -> T,
    {
        self.lock.call_once(|| unsafe {
            *self.ptr.get() = Box::into_raw(Box::new(init()));
        });
        unsafe { &**self.ptr.get() }
    }
}

/// Used to initialize `lock` field in `Lazy` struct.
pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;


#[cfg(test)]
mod test {
    use super::{Lazy, ONCE_INIT};
    use std::thread;
    use std::sync::{Arc, Barrier};
    use std::sync::atomic::{ATOMIC_ISIZE_INIT, AtomicIsize, Ordering};
    use std::cell::UnsafeCell;

    #[test]
    fn many_threads_calling_get() {
        const N_THREADS: usize = 32;
        const N_ITERS_IN_THREAD: usize = 32;
        const N_ITERS: usize = 16;

        static mut LAZY: Lazy<String> = Lazy {
            lock: ONCE_INIT,
            ptr: UnsafeCell::new(0 as *const String),
        };
        static CALL_COUNT: AtomicIsize = ATOMIC_ISIZE_INIT;

        let value = "Hello, world!".to_owned();

        for _ in 0..N_ITERS {
            // Reset mutable state.
            unsafe {
                LAZY = Lazy {
                    lock: ONCE_INIT,
                    ptr: UnsafeCell::new(0 as *const String),
                }
            }
            CALL_COUNT.store(0, Ordering::SeqCst);

            // Create a bunch of threads, all calling .get() at the same time.
            let mut threads = vec![];
            let barrier = Arc::new(Barrier::new(N_THREADS));

            for _ in 0..N_THREADS {
                let cloned_value_thread = value.clone();
                let cloned_barrier = barrier.clone();
                threads.push(thread::spawn(move || {
                    // Ensure all threads start at once to maximise contention.
                    cloned_barrier.wait();
                    for _ in 0..N_ITERS_IN_THREAD {
                        assert_eq!(&cloned_value_thread, unsafe {
                            LAZY.get(|| {
                                CALL_COUNT.fetch_add(1, Ordering::SeqCst);
                                cloned_value_thread.clone()
                            })
                        });
                    }
                }));
            }

            for thread in threads {
                thread.join().unwrap();
            }

            assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
        }
    }
}
