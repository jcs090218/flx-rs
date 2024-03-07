[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/github/tag/jcs090218/flx-rs.svg?label=release&logo=github)](https://github.com/jcs090218/flx-rs/releases/latest)
[![crates.io](https://img.shields.io/crates/v/flx-rs.svg)](https://crates.io/crates/flx-rs)
[![crates.io.d](https://img.shields.io/crates/d/flx-rs)](https://crates.io/crates/flx-rs)

# flx-rs
> Rewrite emacs-flx in Rust for dynamic modules

[![CI](https://github.com/jcs090218/flx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jcs090218/flx-rs/actions/workflows/ci.yml)

## 🔨 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
flx-rs = "0.1.5"
```

This package only exposed one function `flx_rs::score` that takes only two arguments
`(str: &str, query: &str)`.

```rust
use flx_rs;

fn main() {
    let score : Option<flx_rs::Score> = flx_rs::score("switch-to-buffer", "stb");

    println!("Score: {}", score.unwrap().score);   // Score: 237
}
```

See official documentation in https://docs.rs/flx-rs/latest/flx_rs/.

## 📂 Example

- [flx-rs][] - Emacs package uses this as dynamic module

## 🔍 See Also

- [flx][] - Original algorithm in Emacs Lisp
- [FlxCs][] - Rewrite emacs-flx in C#
- [flx-ts][] - Rewrite emacs-flx in TypeScript, with added support for JavaScript
- [flx-c][] - Rewrite emacs-flx in C

## ⚜️ License

`flx-rs` is distributed under the terms of the MIT license.

See [LICENSE](./LICENSE) for details.


<!-- Links -->

[flx]: https://github.com/lewang/flx
[flx-rs]: https://github.com/jcs-elpa/flx-rs
[FlxCs]: https://github.com/jcs090218/FlxCs
[flx-ts]: https://github.com/jcs090218/flx-ts
[flx-c]: https://github.com/jcs090218/flx-c
