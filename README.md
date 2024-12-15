# hmr

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/hmr/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/hmr/actions)
[![Crate](https://img.shields.io/crates/v/hmr.svg?style=flat-square)](https://crates.io/crates/hmr)
[![License](https://img.shields.io/github/license/raviqqe/hmr.svg?style=flat-square)](LICENSE)

Hot Module Reloading (HMR) for Rust.

# Examples

```rust
use hmr::HotModule;
use std::fs::write;
use std::time::sleep;

const PATH: &'static str = "test/foo.txt";
static FOO: HotModule = HotModule::new(PATH);

assert_eq!(&*FOO.load(), "Hello, world!\n".as_bytes());

write(PATH, "Hello, HMR!\n").unwrap();

sleep(1);

assert_eq!(&*FOO.load(), "Hello, HMR!\n".as_bytes());
```

## License

[MIT](LICENSE)
