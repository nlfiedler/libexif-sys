# libexif-sys

Unsafe wrapper around the `libexif` C library. See the `libexif` crate for a safe wrapper with an API more amenable to Rust applications.

## Dependencies

Because this crate is generating bindings for a C/C++ library, there are some additional requirements beyond having the Rust toolchain installed.

* [Rust](https://www.rust-lang.org) stable (2021 edition, version 1.70 or higher)
* [libexif](https://libexif.github.io) (version 0.6.22)
* [Clang](https://clang.llvm.org) (version 5.0 or higher, as dictated by [rust-bindgen](https://github.com/rust-lang/rust-bindgen))
* For now, `pkg-config` is required to facilitate linking with libexif.

## License

While this crate is distributed under the [MIT License](LICENSE), the underlying [libexif](https://libexif.github.io) C library is licensed under the [LGPL version 2.1](http://www.gnu.org/licenses/old-licenses/lgpl-2.1.html).
