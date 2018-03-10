# rust-numerals [![numerals on crates.io](http://meritbadge.herokuapp.com/numerals)](https://crates.io/crates/numerals) [![Build Status](https://travis-ci.org/ogham/rust-numerals.svg?branch=master)](https://travis-ci.org/ogham/rust-numerals)

This is a library for various numeric systems, including ancient, modern, and
just plain strange.

Although the Roman module is likely to be the most popular, I maintain an
interest in the others, so they're all packaged as one!

### [View the Rustdoc](https://docs.rs/numerals)


## Installation

This library uses [Cargo](http://crates.io). Just add `numerals` as a
dependency in your `Cargo.toml`:

```toml
[dependencies]
numerals = "0.1"
```


## Roman

To format a number as Roman numerals, use `Roman::from` to convert it, and the
`UpperHex` formatting trait to format it.

```rust
use numerals::roman::Roman;
let string = format!("{:X}", Roman::from(134));
assert_eq!(string, "CXXXIV");
```
