use std::{fmt, hash::Hash};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Bit {
    Zero,
    One,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

impl From<u64> for Bit {
    fn from(v: u64) -> Self {
        if v == 0 {
            Self::Zero
        } else {
            Self::One
        }
    }
}

impl From<Bit> for u64 {
    fn from(val: Bit) -> Self {
        match val {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value {
            Self::One
        } else {
            Self::Zero
        }
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}
