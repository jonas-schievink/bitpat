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
/// against the specified pattern. The closure always returns a `bool`
/// indicating whether the value matched, but its argument can be inferred to be
/// any integer type.
///
/// If a `bitpat!` is matched against a value that consists of more bits than
/// were specified in the pattern, the pattern is applied to the *least
/// significant* bits in the value and other bits are ignored. In other words,
/// the pattern is padded with `_` symbols.
///
/// Likewise, if the pattern is applied to a value with fewer bits than in the
/// pattern, the pattern will be *truncated* to match against the value. But do
/// note that type inference might infer a larger integer type than you expect,
/// so what seem to be excess bits in the pattern might get matched normally.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// #[macro_use] extern crate bitpat;
///
/// # fn main() {
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
/// # }
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

#[cfg(test)]
mod tests {
    #[test]
    fn mask() {
        assert!(bitpat!(0 0 _ _ 1 _ _ _)(0b00111111));
        assert!(bitpat!(0 0 _ _ 1 _ _ _)(0b00001111));
        assert!(bitpat!(0 0 _ _ 1 _ _ _)(0b00001000));
        assert!(bitpat!(0 0 _ _ 1 _ _ _)(0b00001110));
        assert!(!bitpat!(0 0 _ _ 1 _ _ _)(0b01111111));
        assert!(!bitpat!(0 0 _ _ 1 _ _ _)(0b10111111));
        assert!(!bitpat!(0 0 _ _ 1 _ _ _)(0b00110111));
        for b in 0..=255u8 {
            assert_eq!(bitpat!(1 _ _ _ _ _ _ _)(b), b >= 128);
            assert_eq!(bitpat!(0 _ _ _ _ _ _ _)(b), b < 128);
        }
        for b in 0..=255u8 {
            assert_eq!(bitpat!(_ _ _ _ _ _ _ 1)(b), b & 1 != 0);
            assert_eq!(bitpat!(_ _ _ _ _ _ _ 0)(b), b & 1 == 0);
        }
        for b in 0..=255u8 {
            assert!(bitpat!(_ _ _ _ _ _ _ _)(b));
        }
        for b in 1..=255u8 {
            assert!(!bitpat!(0 0 0 0 0 0 0 0)(b));
        }
    }

    #[test]
    fn mask_too_short() {
        assert!(bitpat!(_ _ _ _)(0b11110000));
        assert!(bitpat!(_ _ _ _)(0b11111111));
        assert!(bitpat!(_ _ _ _)(0b11110001));
        assert!(bitpat!(_ _ _ _)(0b0000));

        assert!(bitpat!(0 0 0 0)(0b11110000));
        assert!(bitpat!(0 0 0 0)(0b1110000));
        assert!(bitpat!(0 0 0 0)(0b110000));
        assert!(bitpat!(0 0 0 0)(0b10000));
        assert!(bitpat!(0 0 0 0)(0b0000));

        assert!(bitpat!(1 1 1 1)(0b11111111));
        assert!(bitpat!(1 1 1 1)(0b1111111));
        assert!(bitpat!(1 1 1 1)(0b111111));
        assert!(bitpat!(1 1 1 1)(0b11111));
        assert!(bitpat!(1 1 1 1)(0b1111));
    }

    #[test]
    fn mask_too_long() {
        assert!(bitpat!(_   _ _ _ _ _ _ _ _)(0b11110000u8));
        assert!(bitpat!(0   _ _ _ _ _ _ _ _)(0b11110000u8));
        assert!(bitpat!(1   _ _ _ _ _ _ _ _)(0b11110000u8));
        assert!(bitpat!(1   1 _ _ _ _ _ _ _)(0b11110000u8));
        assert!(bitpat!(0   1 _ _ _ _ _ _ _)(0b11110000u8));
        assert!(bitpat!(1   0 _ _ _ _ _ _ _)(0b01110000u8));
        assert!(bitpat!(0   0 _ _ _ _ _ _ _)(0b01110000u8));
    }
}
