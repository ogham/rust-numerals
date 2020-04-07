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


/// An individual Roman numeral, without a given position.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Numeral {
    I, V, X, L, C, D, M,
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
    pub fn from_char(input: char) -> Option<Numeral> {
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
    pub fn parse(input: &str) -> Option<Roman> {
        let mut numerals = Vec::new();

        for c in input.chars() {
            match Numeral::from_char(c) {
                Some(numeral)  => numerals.push(numeral),
                None           => return None,
            }
        }

        Some(Roman { numerals })
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
}

impl fmt::LowerHex for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in self.numerals.iter() {
            write!(f, "{}", n.ascii_lower())?
        }
        Ok(())
    }
}

impl fmt::UpperHex for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in self.numerals.iter() {
            write!(f, "{}", n.ascii_upper())?;
        }

        Ok(())
    }
}

impl From<Vec<Numeral>> for Roman {
    fn from(input: Vec<Numeral>) -> Roman {
        Roman { numerals: input }
    }
}

impl From<i16> for Roman {
    fn from(mut number: i16) -> Roman {
        assert!(number > 0);
        let mut vec = Vec::new();

        for &(secondary, primary) in &[ (C, M), (C, D),
                                        (X, C), (X, L),
                                        (I, X), (I, V) ] {

            while number >= primary.value() {
                number -= primary.value();
                vec.push(primary);
            }

            let difference = primary.value() - secondary.value();
            if number >= difference {
                number -= difference;
                vec.push(secondary);
                vec.push(primary);
            }
        }

        while number > 0 {
            number -= 1;
            vec.push(I);
        }

        Roman { numerals: vec }
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
	}
}
