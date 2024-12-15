//! Hot Module Reloading.

use std::{
    fs::read,
    sync::{LazyLock, RwLock},
};

/// A hot reloaded module.
pub struct HotModule {
    current: LazyLock<Vec<u8>>,
    next: RwLock<Option<Vec<u8>>>,
}

impl HotModule {
    /// Loads a module.
    pub fn load(&self) -> &[u8] {
        if let Ok(foo) = self.next.try_read() {
            self.current = foo;
        }
        *self.lock
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
