//! Allows bit-level matching against values.
//!
//! Please refer to the [`bitpat!`] macro for details.
//!
//! [`bitpat!`]: macro.bitpat.html

#![doc(html_root_url = "https://docs.rs/bitpat/0.1.0")]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

/// Builds a closure for bit-level matching of a value.
///
/// # Syntax
///
/// The `bitpat!` macro takes a list of `1`, `0` or `_` tokens, each of which
/// correspond to a bit in the values it will match against. A `1` token matches
/// `1` bits, a `0` token matches `0` bits, and a `_` token matches either one.
///
/// `bitpat!` expands to a closure that takes a value that is then matched
/// against the specified pattern.
///
/// If a `bitpat!` is matched against a value that consists of more bits than
/// were specified in the pattern, the pattern is applied to the *least
/// significant* bits in the value and other bits are ignored. In other words,
/// the pattern is padded with `_` symbols.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate bitpat;
///
/// // `0` patterns must always be 0, while `_` patterns don't matter.
/// assert!( bitpat!(0 0 0 0 _ _ _ _)(0b00000000u8));
/// assert!( bitpat!(0 0 0 0 _ _ _ _)(0b00001111u8));
/// assert!( bitpat!(0 0 0 0 _ _ _ _)(0b00001000u8));
/// assert!( bitpat!(0 0 0 0 _ _ _ _)(0b00000001u8));
///
/// assert!(!bitpat!(0 0 0 0 _ _ _ _)(0b10000000u8));
/// assert!(!bitpat!(0 0 0 0 _ _ _ _)(0b11110000u8));
/// assert!(!bitpat!(0 0 0 0 _ _ _ _)(0b11111111u8));
/// assert!(!bitpat!(0 0 0 0 _ _ _ _)(0b00011111u8));
///
/// // `1` patterns work analogously
/// assert!( bitpat!(1 1 1 _ _ 0 0 0)(0b11100000u8));
/// assert!( bitpat!(1 1 1 _ _ 0 0 0)(0b11110000u8));
/// assert!( bitpat!(1 1 1 _ _ 0 0 0)(0b11111000u8));
/// assert!( bitpat!(1 1 1 _ _ 0 0 0)(0b11101000u8));
///
/// assert!(!bitpat!(1 1 1 _ _ 0 0 0)(0b00000000u8));
/// assert!(!bitpat!(1 1 1 _ _ 0 0 0)(0b11111111u8));
/// assert!(!bitpat!(1 1 1 _ _ 0 0 0)(0b11111100u8));
/// assert!(!bitpat!(1 1 1 _ _ 0 0 0)(0b00001111u8));
/// assert!(!bitpat!(1 1 1 _ _ 0 0 0)(0b11000000u8));
/// ```
#[macro_export]
macro_rules! bitpat {
    // no more parts left, done building the masks
    ( @build $relevant:tt $ones:tt [] ) => {
        |value| value & ($relevant) == ($ones)
    };

    // incrementally build the masks, shifting them to the left and adding
    // another bit, `$next`, on the right
    ( @build $relevant:tt $ones:tt [$next:tt $($rest:tt)*] ) => {
        bitpat!(@build ($relevant << 1 | bitpat!(@relevant $next)) ($ones << 1 | bitpat!(@is_one $next)) [$($rest)*])
    };

    // Whether a bit in the value is relevant for the match
    ( @relevant _ ) => { 0 };
    ( @relevant 0 ) => { 1 };
    ( @relevant 1 ) => { 1 };

    // Whether the bit must be 1
    ( @is_one _ ) => { 0 };
    ( @is_one 0 ) => { 0 };
    ( @is_one 1 ) => { 1 };

    // Entry point
    ( $($part:tt)+ ) => {bitpat!(@build 0 0 [$($part)+])};
}
