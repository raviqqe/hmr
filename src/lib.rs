//! Hot Module Reloading.

use std::{fs::read, sync::LazyLock};

/// A hot reloaded module.
pub struct HotModule {
    lock: LazyLock<&'static [u8]>,
}

impl HotModule {
    /// Creates a hot reloaded module.
    pub const fn new(path: &'static str) -> Self {
        Self {
            lock: LazyLock::new(|| read(path).expect("readable file")),
        }
    }
}

#[macro_export]
macro_rules! load {
    ($name:ident, $path:literal) => {
        static $name: HotModule = HotModule {
            lock: LazyLock::new(|| read(path).expect("readable file")),
        };
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        static FOO: LazyLock<&'static [u8]> = load("./lib.rs");

        assert_eq!(&*FOO, &[]);
    }
}
