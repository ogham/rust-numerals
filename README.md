This is a library for various numeric systems, including ancient, modern, and
just plain strange.

Although the Roman module is likely to be the most popular, I maintain an
interest in the others, so they're all packaged as one!

### [View the Rustdoc](http://bsago.me/doc/numerals)


## Installation

This library uses [Cargo](http://crates.io). Just add `numerals` as a
dependency in your `Cargo.toml`:

```toml
[dependencies]
numerals = "*"
```

or

```toml
[dependencies]
git = "https://github.com/ogham/rust-numerals.git"
```


## Roman

```rust
use numerals::roman::Roman;
let string = format!("{:X}", Roman::from(134));
assert_eq!(string, "CXXXIV");
```
