use std::{fmt, hash::Hash};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

type Input = (Bit, Bit);
type GateTable = [(Input, Bit); 4];

pub trait Gate {
    fn table(&self) -> &GateTable;
}

impl fmt::Display for dyn Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table = self.table();
        writeln!(f, "Truth Table for {}:", std::any::type_name::<Self>())?;
        writeln!(f, " A  | B  | Output")?;
        writeln!(f, "----|----|-------")?;
        for ((a, b), output) in table {
            writeln!(f, " {}  | {}  | {}", a, b, output)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AndGate;

#[derive(Debug, Clone)]
pub struct OrGate;

#[derive(Debug, Clone)]
pub struct XorGate;

#[derive(Debug, Clone)]
pub struct NandGate;

impl Gate for AndGate {
    fn table(&self) -> &GateTable {
        &[
            ((Bit::Zero, Bit::Zero), Bit::Zero),
            ((Bit::Zero, Bit::One), Bit::Zero),
            ((Bit::One, Bit::Zero), Bit::Zero),
            ((Bit::One, Bit::One), Bit::One),
        ]
    }
}

impl Gate for OrGate {
    fn table(&self) -> &GateTable {
        &[
            ((Bit::Zero, Bit::Zero), Bit::Zero),
            ((Bit::Zero, Bit::One), Bit::One),
            ((Bit::One, Bit::Zero), Bit::One),
            ((Bit::One, Bit::One), Bit::One),
        ]
    }
}

impl Gate for XorGate {
    fn table(&self) -> &GateTable {
        &[
            ((Bit::Zero, Bit::Zero), Bit::Zero),
            ((Bit::Zero, Bit::One), Bit::One),
            ((Bit::One, Bit::Zero), Bit::One),
            ((Bit::One, Bit::One), Bit::Zero),
        ]
    }
}

impl Gate for NandGate {
    fn table(&self) -> &GateTable {
        &[
            ((Bit::Zero, Bit::Zero), Bit::One),
            ((Bit::Zero, Bit::One), Bit::One),
            ((Bit::One, Bit::Zero), Bit::One),
            ((Bit::One, Bit::One), Bit::Zero),
        ]
    }
}
