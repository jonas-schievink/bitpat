# bitpat - Bit-level pattern matching

[![crates.io](https://img.shields.io/crates/v/bitpat.svg)](https://crates.io/crates/bitpat)
[![docs.rs](https://docs.rs/bitpat/badge.svg)](https://docs.rs/bitpat/)
[![Build Status](https://travis-ci.org/jonas-schievink/bitpat.svg?branch=master)](https://travis-ci.org/jonas-schievink/bitpat)

This crate provides the `bitpat!` macro, which can match a value against a bit
pattern. This is useful, for example, for low-level code that inspects
individual bits in data such as registers or machine instructions.

Please refer to the [changelog](CHANGELOG.md) to see what changed in the last
releases.

## Usage

Start by adding an entry to your `Cargo.toml`:

```toml
[dependencies]
bitpat = "0.1.0"
```

Then import the crate into your Rust code:

```rust
extern crate bitpat;
```
