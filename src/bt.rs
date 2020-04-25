//! [Balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary)

#![allow(missing_docs)]  // TODO

use std::convert::From;
use std::fmt;

use self::Trit::*;


#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Trit {
    Minus, Zero, Plus
}

impl Trit {
    fn value(self) -> i64 {
        match self {
            Minus => -1,
            Zero  => 0,
            Plus  => 1,
        }
    }

    fn ascii(self) -> char {
        match self {
            Minus => '-',
            Zero  => '0',
            Plus  => '+',
        }
    }

    pub fn from_char(input: char) -> Option<Self> {
        match input {
            '-' => Some(Minus),
            '0' => Some(Zero),
            '+' => Some(Plus),
            _   => None,
        }
    }
}


pub struct BalancedTernary {
    trits: Vec<Trit>,
}

impl BalancedTernary {
    pub fn parse(input: &str) -> Option<Self> {
        let mut trits = Vec::new();

        for c in input.chars() {
            match Trit::from_char(c) {
                Some(trit)  => trits.push(trit),
                None        => return None,
            }
        }

        Some(Self { trits })
    }

    pub fn value(&self) -> i64 {
        self.trits.iter().fold(0, |sum, trit| sum * 3 + trit.value())
    }
}

impl fmt::Display for BalancedTernary {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for t in &self.trits {
		    write!(f, "{}", t.ascii())?;
		}

		Ok(())
	}
}

impl From<i64> for BalancedTernary {
    fn from(mut input: i64) -> Self {
        let mut trits = Vec::new();

        while input != 0 {
            match input % 3 {
                0 => { trits.push(Zero);  input /= 3; },
                1 => { trits.push(Plus);  input /= 3; },
                2 => { trits.push(Minus); input += 1; input /= 3; },
                _ => unreachable!(),
            }
        }

        trits.reverse();

        Self { trits }
    }
}


#[cfg(test)]
mod test {
    use super::BalancedTernary;

	#[test]
	fn test_many_numbers() {
		for i in 0 .. 4321 {
            assert_eq!(i, BalancedTernary::from(i).value());
		}
	}
}
