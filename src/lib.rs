//! Hot Module Reloading.

use std::{fs::read, sync::LazyLock};

/// A hot reloaded module.
pub struct HotModule {
    lock: LazyLock<&'static [u8]>,
}

impl HotModule {
    /// Creates a hot reloaded module.
    pub fn new() -> Self {
        LazyLock::new(|| read(path).expect("readable file"))
    }
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
