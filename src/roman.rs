

use std::convert::From;
use std::fmt;

use self::Numeral::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Numeral {
    I, V, X, L, C, D, M,
}

impl Numeral {
    pub fn value(&self) -> i32 {
        match *self {
            I =>    1,  V =>   5,
            X =>   10,  L =>  50,
            C =>  100,  D => 500,
            M => 1000,
        }
    }

    fn ascii_upper(&self) -> char {
        match *self {
            I => 'I',  V => 'V',  X => 'X',  L => 'L',
            C => 'C',  D => 'D',  M => 'M',
        }
    }

    fn ascii_lower(&self) -> char {
        match *self {
            I => 'i',  V => 'v',  X => 'x',  L => 'l',
            C => 'c',  D => 'd',  M => 'm',
        }
    }

    pub fn from_char(input: char) -> Option<Numeral> {
        match input {
            'I' | 'i' => Some(I),  'V' | 'v' => Some(V),
            'X' | 'x' => Some(X),  'L' | 'l' => Some(L),
            'C' | 'c' => Some(C),  'D' | 'd' => Some(D),
            'M' | 'm' => Some(M),      _     => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Roman {
    numerals: Vec<Numeral>,
}

impl Roman {
    pub fn parse(input: &str) -> Option<Roman> {
        let mut numerals = Vec::new();

        for c in input.chars() {
            match Numeral::from_char(c) {
                Some(numeral)  => numerals.push(numeral),
                None           => return None,
            }
        }

        Some(Roman { numerals: numerals })
    }

    pub fn value(&self) -> i32 {
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
            try!(write!(f, "{}", n.ascii_lower()))
        }
        Ok(())
    }
}

impl fmt::UpperHex for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in self.numerals.iter() {
            try!(write!(f, "{}", n.ascii_upper()))
        }
        Ok(())
    }
}

impl From<i32> for Roman {
    fn from(mut number: i32) -> Roman {
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
