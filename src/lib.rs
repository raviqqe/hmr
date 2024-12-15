//! Hot Module Reloading.

use std::{
    fs::read,
    mem::take,
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

impl HotModule {
    /// Crates a new hot reloaded module.
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            current: RwLock::new(Vec::new()),
            next: RwLock::new(None),
            watcher: OnceLock::new(),
        }
    }

    /// Loads a module.
    pub fn load(&'static self) -> RwLockReadGuard<'static, Vec<u8>> {
        self.watcher.get_or_init(|| {
            let mut lock = self.current.write().expect("write lock");
            *lock = read(self.path).expect("readable file");

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

        if let Ok(mut content) = self.next.try_write() {
            if let Some(content) = take(&mut *content) {
                if let Ok(mut current) = self.current.try_write() {
                    *current = content;
                }
            }
        }

        self.current.read().expect("read lock")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        static FOO: HotModule = HotModule::new("test/foo.txt");

        assert_eq!(*FOO.load(), "Hello, world!\n".as_bytes());
    }
}
