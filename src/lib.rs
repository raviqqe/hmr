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
    pub const fn new(path: &'static str) -> Self {
        Self {
            current: RwLock::new(Vec::new()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        static FOO: HotModule = HotModule::new("lib.rs");

        assert_eq!(*FOO.load(), &[]);
    }
}
