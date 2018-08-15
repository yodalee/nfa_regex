use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug,Clone,PartialEq)]
pub enum FARuleData {
    Char {character: char},
    Range {start: char, end: char},
}

impl FARuleData {
    pub fn char(c: char) -> Self {
        FARuleData::Char { character: c }
    }

    pub fn range(start: char, end: char) -> Self {
        FARuleData::Range { start: start, end: end }
    }

    pub fn applies_to(&self, c: &char) -> bool {
        match self {
            FARuleData::Char { character } => character == c,
            FARuleData::Range { start, end } => start <= c && c <= end
        }
    }
}

impl Display for FARuleData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FARuleData::Char { character } => write!(f, "{}", character),
            FARuleData::Range { start, end } => write!(f, "{}-{}", start, end),
        }
    }
}

