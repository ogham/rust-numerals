# rust-numerals [![numerals on crates.io](http://meritbadge.herokuapp.com/numerals)](https://crates.io/crates/numerals) [![Build Status](https://travis-ci.org/ogham/rust-numerals.svg?branch=master)](https://travis-ci.org/ogham/rust-numerals)

This is a library for various numeric systems, including ancient, modern, and just plain strange.

Although the Roman module is likely to be the most popular, I maintain an interest in the others, so they’re all packaged as one!

### [View the Rustdoc](https://docs.rs/numerals)


## Installation

This library works with [Cargo](https://crates.io).
Add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
numerals = "0.1"
```

The earliest version of Rust that this crate is tested against is [Rust v1.31.0](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html).


## Roman Numerals

To format a number as Roman numerals, use `Roman::from` to convert it, and the `UpperHex` formatting trait to format it.

```rust
use numerals::roman::Roman;

let string = format!("{:X}", Roman::from(134));
assert_eq!(string, "CXXXIV");
```
