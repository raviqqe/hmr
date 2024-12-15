//! Hot Module Reloading.

use std::{
    fs::read,
    mem::take,
    sync::{RwLock, RwLockReadGuard},
};

/// A hot reloaded module.
pub struct HotModule {
    current: RwLock<Vec<u8>>,
    next: RwLock<Option<Vec<u8>>>,
}

impl HotModule {
    /// Loads a module.
    pub fn load(&self) -> RwLockReadGuard<Vec<u8>> {
        if let Ok(mut content) = self.next.try_write() {
            if let Some(content) = take(&mut *content) {
                if let Ok(mut current) = self.current.try_write() {
                    *current = content;
                }
            }
        }

        self.current.read().expect("lock")
    }
}

#[macro_export]
macro_rules! load {
    ($name:ident, $path:literal) => {
        mod $name {
            use super::*;

            static CONTENT: RwLock<Vec<u8>> = RwLock::new(Vec::new());
        }

        static $name: HotModule = HotModule {
            lock: LazyLock::new(|| {
                let lock = CONTENT.write().expect("write lock");
                *lock = read(path).expect("readable file");
                binary.read()
            }),
        };
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        load!(FOO, "lib.rs");

        assert_eq!(FOO.load(), &[]);
    }
}
