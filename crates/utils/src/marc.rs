use std::sync::{Arc, Mutex};

use super::prelude::Result;

/// A thread-safe reference-counted pointer to a value of type T, with a description for debugging.
/// A replacement for `Arc<Mutex<T>>` that includes a method for locking with error handling and
/// description for easier debugging.
///
/// # Example
/// ```rust
/// use utils::prelude::*;
///
/// fn main() -> anyhow::Result<()> {
///     let data = "Hello, world!";
///     let data_prt = MArc::new(data.to_string(), "Shared data");
///
///     {
///         let mut lock = data_prt.lock()?;
///         *lock = "Hello, Rust!".to_string();
///     };
///
///     {
///         let lock = data_prt.lock()?;
///         println!("Locked data: {}", *lock);
///     };
///
///     Ok(())
/// }
/// ```
pub struct MArc<T> {
    inner: Arc<Mutex<T>>,
    description: String,
}

impl<T> Clone for MArc<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            description: self.description.clone(),
        }
    }
}

impl<T> MArc<T> {
    /// Creates a new `MArc<T>` with the given value and description.
    pub fn new(value: T, description: &str) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
            description: description.to_string(),
        }
    }

    /// Locks the inner `Mutex<T>` and returns a `Ok(MutexGuard<T>)`. Returns an `Err` if the lock is poisoned,
    pub fn lock(&self) -> Result<std::sync::MutexGuard<'_, T>> {
        match self.inner.lock() {
            Ok(guard) => Ok(guard),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to lock MArc ({}): {}",
                self.description,
                e
            )),
        }
    }

    /// Locks the inner `Mutex<T>` and returns a `MutexGuard<T>`. Panics if the lock is poisoned.
    pub fn lock_panic(&self) -> std::sync::MutexGuard<'_, T> {
        #[allow(clippy::unwrap_used)]
        self.lock().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::MArc;

    #[test]
    fn lock() {
        let data = "Hello, world!";
        let data_prt = MArc::new(data.to_string(), "Shared data");

        {
            let lock = data_prt.lock().unwrap();
            println!("Locked data: {}", *lock);
            assert_eq!(*lock, "Hello, world!");
        };
    }

    #[test]
    fn lock_mut() {
        let data = "Hello, world!";
        let data_prt = MArc::new(data.to_string(), "Shared data");

        {
            let mut lock = data_prt.lock().unwrap();
            *lock = "Hello, Rust!".to_string();
        };

        {
            let lock = data_prt.lock().unwrap();
            println!("Locked data: {}", *lock);
            assert_eq!(*lock, "Hello, Rust!");
        };
    }

    #[test]
    fn clone() {
        let data = "Hello, world!";
        let data_prt = MArc::new(data.to_string(), "Shared data");
        let data_prt2 = data_prt.clone();

        {
            let lock = data_prt.lock().unwrap();
            println!("Locked data: {}", *lock);
            assert_eq!(*lock, "Hello, world!");
        };

        {
            let lock = data_prt2.lock().unwrap();
            println!("Locked data: {}", *lock);
            assert_eq!(*lock, "Hello, world!");
        };
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Failed to lock MArc (Shared data): poisoned lock: another task failed inside"
    )]
    fn poisoned() {
        let data = "Hello, world!";
        let data_prt = MArc::new(data.to_string(), "Shared data");

        let result = std::panic::catch_unwind(|| {
            let _lock = data_prt.lock().unwrap();
            panic!("Simulated panic while holding the lock");
        });

        assert!(
            result.is_err(),
            "Expected an error, but code executed successfully"
        );

        {
            let lock = data_prt.lock().unwrap();
            println!("Locked data after panic: {}", *lock);
            assert_eq!(*lock, "Hello, world!");
        };
    }
}
