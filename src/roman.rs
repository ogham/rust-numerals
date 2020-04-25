//! [Ancient Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals)
//!
//! This is a library for converting between computer integers, Roman
//! numerals, and ASCII strings.
//!
//! Operations are done by converting your input into a `Roman` value, then
//! calling functions on it. For example:
//!
//! ```
//! use numerals::roman::Roman;
//!
//! let string = format!("{:X}", Roman::from(134));
//! assert_eq!(string, "CXXXIV");
//! ```
//!
//!
//! Converting numbers to Roman
//! ---------------------------
//!
//! The `From` function in `std::convert` can turn either an `i16`, or a
//! vector of `Numeral` values, into a `Roman` value.
//!
//! ```
//! use numerals::roman::{Roman, Numeral::{I, V, X}};
//!
//! let input    = Roman::from(27);
//! let expected = Roman::from(vec![ X, X, V, I, I ]);
//! assert_eq!(expected, input);
//! ```
//!
//!
//! Converting Roman to numbers
//! ---------------------------
//!
//! The `value` function translates a sequence of numerals into their computer
//! value equivalent.
//!
//! ```
//! use numerals::roman::{Roman, Numeral::{I, V, X}};
//!
//! let input = Roman::from(vec![ X, X, V, I, I ]).value();
//! assert_eq!(27, input);
//! ```
//!
//!
//! Converting strings to Roman
//! ---------------------------
//!
//! You can translate an existing sequence of characters with the `parse`
//! constructor, which scans an input string, returning `None` if it
//! encounters a character with no Roman meaning.
//!
//! It accepts both uppercase and lowercase ASCII characters.
//!
//! ```
//! use numerals::roman::{Roman, Numeral::{I, V, X}};
//!
//! let input    = Roman::parse("XXVII").unwrap();
//! let expected = Roman::from(vec![ X, X, V, I, I ]);
//! assert_eq!(expected, input);
//! ```
//!
//!
//! Converting Roman to strings
//! ---------------------------
//!
//! There are two ways to convert numerals into strings:
//!
//! - For uppercase, use the `UpperHex` trait with the `{:X}` format string.
//! - For lowercase, use the `LowerHex` trait with the `{:x}` format string.
//!
//! ```
//! use numerals::roman::{Roman, Numeral::{I, V, X}};
//!
//! let input = format!("{:X}", Roman::from(vec![ X, X, V, I, I ]));
//! assert_eq!("XXVII", input);
//! ```
//!
//!
//! Limitations
//! -----------
//!
//! - The `Roman::from(i16)` function will **panic when given zero or a
//!   negative number!** The Romans had the *concept* of zero, but no numeral
//!   for it, so it’s not relevant here. Be sure to check your input values.
//! - Similarly, there is no *common* way to handle numbers in the tens of
//!   thousands, which is why this library uses `i16`-sized integers. Numbers
//!   in the tens of thousands will work, but will be prefixed with many, many
//!   `M`s.

use std::convert::From;
use std::fmt;

use self::Numeral::*;


/// An individual Roman numeral, without a position.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Numeral {

    /// The ‘Ⅰ’ numeral, representing 1.
    I,

    /// The ‘Ⅴ’ numeral, representing 5.
    V,

    /// The ‘Ⅹ’ numeral, representing 10.
    X,

    /// The ‘Ⅼ’ numeral, representing 50.
    L,

    /// The ‘C’ numeral, representing 100.
    C,

    /// The ‘Ⅾ’ numeral, representing 500.
    D,

    /// The ‘Ⅿ’ numeral, representing 1000.
    M,
}

impl Numeral {
    /// The value that this numeral is “worth”. “Worth” is in scare quotes, as
    /// the value can change depending on its position in the string.
    pub fn value(self) -> i16 {
        match self {
            I =>    1,  V =>   5,
            X =>   10,  L =>  50,
            C =>  100,  D => 500,
            M => 1000,
        }
    }

    fn ascii_upper(self) -> char {
        match self {
            I => 'I',  V => 'V',  X => 'X',  L => 'L',
            C => 'C',  D => 'D',  M => 'M',
        }
    }

    fn ascii_lower(self) -> char {
        match self {
            I => 'i',  V => 'v',  X => 'x',  L => 'l',
            C => 'c',  D => 'd',  M => 'm',
        }
    }

    /// Turn an individual character into its numeral equivalent, if there is
    /// one. Returns `None` otherwise.
    ///
    /// This accepts either uppercase or lowercase ASCII characters.
    pub fn from_char(input: char) -> Option<Self> {
        match input {
            'I' | 'i' => Some(I),  'V' | 'v' => Some(V),
            'X' | 'x' => Some(X),  'L' | 'l' => Some(L),
            'C' | 'c' => Some(C),  'D' | 'd' => Some(D),
            'M' | 'm' => Some(M),      _     => None,
        }
    }
}


/// A sequence of Roman numerals.
#[derive(PartialEq, Debug)]
pub struct Roman {
    numerals: Vec<Numeral>,
}

impl Roman {

    /// Parses a string of characters into a sequence of numerals. Returns
    /// `None` if there’s a character in the input string that doesn’t map to
    /// a numeral.
    ///
    /// This accepts either uppercase or lowercase ASCII characters.
    pub fn parse(input: &str) -> Option<Self> {
        let mut numerals = Vec::new();

        for c in input.chars() {
            numerals.push(Numeral::from_char(c)?);
        }

        Some(Self { numerals })
    }

    /// Converts this string of numerals into a `i32` actual number.
    ///
    /// # Panics
    ///
    /// - This function panics when passed in a negative number or zero, as
    ///   the Romans didn’t have a way to write those down!
    pub fn value(&self) -> i16 {
        let mut total = 0;
        let mut max = 0;

        for n in self.numerals.iter().map(|n| n.value()).rev() {
            total += if n >= max { n } else { -n };

            if max < n {
                max = n;
            }
        }

        total
    }

    /// Converts this string of numerals into a `i32` actual number.
    /// Unlike `value`, this returns `None` on numbers that can't be converted to an `i32`.
    pub fn value_checked(&self) -> Option<i16> {
        let mut total: i16 = 0;
        let mut max = 0;

        for n in self.numerals.iter().map(|n| n.value()).rev() {
            let amount_to_add = if n >= max { n } else { -n };
            total = total.checked_add(amount_to_add)?;

            if max < n {
                max = n;
            }
        }

        Some(total)
    }
}

impl fmt::LowerHex for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in &self.numerals {
            write!(f, "{}", n.ascii_lower())?
        }

        Ok(())
    }
}

impl fmt::UpperHex for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in &self.numerals {
            write!(f, "{}", n.ascii_upper())?;
        }

        Ok(())
    }
}

impl From<Vec<Numeral>> for Roman {
    fn from(numerals: Vec<Numeral>) -> Self {
        Self { numerals }
    }
}

impl From<i16> for Roman {
    fn from(mut number: i16) -> Self {
        assert!(number > 0);
        let mut numerals = Vec::new();

        for &(secondary, primary) in &[ (C, M), (C, D),
                                        (X, C), (X, L),
                                        (I, X), (I, V) ] {

            while number >= primary.value() {
                number -= primary.value();
                numerals.push(primary);
            }

            let difference = primary.value() - secondary.value();
            if number >= difference {
                number -= difference;
                numerals.push(secondary);
                numerals.push(primary);
            }
        }

        while number > 0 {
            number -= 1;
            numerals.push(I);
        }

        Self { numerals }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_many_numbers() {
        for i in 1 .. 4321 {
            assert_eq!(i, Roman::from(i).value())
        }
        for i in 1 .. 4321 {
            assert_eq!(Some(i), Roman::from(i).value_checked())
        }
    }

    #[test]
    fn test_big_numbers() {
        for i in 32700 .. 32767 {
            assert_eq!(i, Roman::from(i).value());
        }
        for i in 32700 .. 32767 {
            assert_eq!(Some(i), Roman::from(i).value_checked());
        }
    }

    #[test]
    fn value_checked_err_on_large() {
        assert_eq!(
            Roman::parse("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM")
                .unwrap()
                .value_checked(),
            None
        );
    }

    #[test]
    #[should_panic]
    fn value_panic_on_large() {
        Roman::parse("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM")
            .unwrap()
            .value();
    }
}
