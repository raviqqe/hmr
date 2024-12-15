//! Hot Module Reloading.

use std::{
    fs::read,
    mem::take,
    ops::Deref,
    sync::{OnceLock, RwLock, RwLockReadGuard},
};

use notify::{Event, RecommendedWatcher, Watcher};

/// A hot reloaded module.
pub struct HotModule {
    path: &'static str,
    current: RwLock<Vec<u8>>,
    next: RwLock<Option<Vec<u8>>>,
    watcher: OnceLock<RecommendedWatcher>,
}

/// A read guard of a hot reloaded module.
pub struct Guard(RwLockReadGuard<'static, Vec<u8>>);

impl Deref for Guard {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HotModule {
    /// Crates a hot reloaded module.
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            current: RwLock::new(Vec::new()),
            next: RwLock::new(None),
            watcher: OnceLock::new(),
        }
    }

    /// Loads a module content.
    pub fn load(&'static self) -> Guard {
        self.watcher.get_or_init(|| {
            let mut lock = self.current.write().unwrap();
            *lock = read(self.path).unwrap();

            RecommendedWatcher::new(
                |result| {
                    if let Ok(Event { kind, .. }) = result {
                        if kind.is_modify() {
                            if let Ok(mut content) = self.next.write() {
                                *content = Some(read(self.path).unwrap());
                            }
                        }
                    }
                },
                Default::default(),
            )
            .unwrap()
        });

        // All lock functions used here must be asynchronous.
        if let Ok(mut content) = self.next.try_write() {
            if let Some(content) = take(&mut *content) {
                if let Ok(mut current) = self.current.try_write() {
                    *current = content;
                }
            }
        }

        Guard(self.current.read().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        static FOO: HotModule = HotModule::new("test/foo.txt");

        assert_eq!(&*FOO.load(), "Hello, world!\n".as_bytes());
    }
}
