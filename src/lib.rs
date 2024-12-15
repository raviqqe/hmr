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
    pub const fn new(path: &str) -> Self {
        let content = read(path).expect("readable file");

        Self {
            current: RwLock::new(read(path).expect("readable file")),
            next: RwLock::new(None),
        }
    }

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

        static $name: HotModule = HotModule {};
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
