# hmr

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/hmr/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/hmr/actions)
[![Crate](https://img.shields.io/crates/v/hmr.svg?style=flat-square)](https://crates.io/crates/hmr)
[![License](https://img.shields.io/github/license/raviqqe/hmr.svg?style=flat-square)](LICENSE)

Hot Module Reloading (HMR) for Rust.

# Examples

```rust
use hmr::HotModule;

static FOO: HotModule = HotModule::new("test/foo.txt");

assert_eq!(&*FOO.load(), "Hello, world!\n".as_bytes());
```

## License

[MIT](LICENSE)
