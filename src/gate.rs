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

struct BitArray<const I: usize> {
    inner: [Bit; I],
}

impl<const I: usize> BitArray<I> {
    pub fn new(inner: [Bit; I]) -> Self {
        Self { inner }
    }
}

impl<const I: usize> From<usize> for BitArray<I> {
    fn from(value: usize) -> Self {
        let mut res = [Bit::Zero; I];
        let mut v = value;
        for i in (0..I).rev() {
            let bit: Bit = ((v % 2) as u64).into();
            v >>= 1;
            res[i] = bit;
        }
        Self::new(res)
    }
}

impl<const I: usize> From<BitArray<I>> for usize {
    fn from(value: BitArray<I>) -> Self {
        let mut sum: usize = 0;
        for bit in value.inner {
            sum <<= 1;
            let v: u64 = bit.into();
            sum |= v as usize;
        }
        sum
    }
}

// TODO: we won't even need to store the inputs, the inputs can just be array index
pub type Table<const I: usize> = [([Bit; I], Bit); 1 << I];

pub struct Gate<const I: usize>
where
    [(); 1 << I]:,
{
    table: Table<I>,
}

impl<const I: usize> Gate<I>
where
    [(); 1 << I]:,
{
    pub fn from_table(mut table: [([Bit; I], Bit); 1 << I]) -> Self {
        // Ensure sorted, TODO: use sorted array
        table.sort_by(|a, b| a.0.cmp(&b.0));
        Self { table }
    }

    pub fn evaluate(&self, input: &[Bit; I]) -> Bit {
        let idx = self
            .table
            .binary_search_by(|(i, _)| i.cmp(input))
            .expect("Table does not have input");
        self.table[idx].1
    }

    pub fn table(&self) -> &Table<I> {
        &self.table
    }

    // pub fn stack<const J: usize>(&self, other: Gate<J>, out_gate: Gate<2>) -> Gate<{ I + J }> {}
}

impl<const I: usize> fmt::Display for Gate<I>
where
    [(); 1 << I]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (input, output) in self.table.iter() {
            let input_str = input
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(f, "({}) -> {}", input_str, output)?;
        }
        Ok(())
    }
}

pub const ANDGATE: Gate<2> = Gate {
    table: [
        ([Bit::Zero, Bit::Zero], Bit::Zero),
        ([Bit::Zero, Bit::One], Bit::Zero),
        ([Bit::One, Bit::Zero], Bit::Zero),
        ([Bit::One, Bit::One], Bit::One),
    ],
};

pub const ORGATE: Gate<2> = Gate {
    table: [
        ([Bit::Zero, Bit::Zero], Bit::Zero),
        ([Bit::Zero, Bit::One], Bit::One),
        ([Bit::One, Bit::Zero], Bit::One),
        ([Bit::One, Bit::One], Bit::One),
    ],
};

pub const XORGATE: Gate<2> = Gate {
    table: [
        ([Bit::Zero, Bit::Zero], Bit::Zero),
        ([Bit::Zero, Bit::One], Bit::One),
        ([Bit::One, Bit::Zero], Bit::One),
        ([Bit::One, Bit::One], Bit::Zero),
    ],
};

pub const NANDGATE: Gate<2> = Gate {
    table: [
        ([Bit::Zero, Bit::Zero], Bit::One),
        ([Bit::Zero, Bit::One], Bit::One),
        ([Bit::One, Bit::Zero], Bit::One),
        ([Bit::One, Bit::One], Bit::Zero),
    ],
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitarray_to_usize() {
        // Test Case 1: Simple case
        let bit_array = BitArray::new([Bit::One, Bit::Zero, Bit::One]); // Binary: 101
        let num: usize = bit_array.into();
        assert_eq!(num, 5);

        // Test Case 2: All zeros
        let bit_array = BitArray::new([Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero]); // Binary: 0000
        let num: usize = bit_array.into();
        assert_eq!(num, 0);

        // Test Case 3: All ones
        let bit_array = BitArray::new([Bit::One, Bit::One, Bit::One, Bit::One]); // Binary: 1111
        let num: usize = bit_array.into();
        assert_eq!(num, 15);

        // Test Case 4: Mixed bits
        let bit_array = BitArray::new([Bit::Zero, Bit::One, Bit::One, Bit::Zero]); // Binary: 0110
        let num: usize = bit_array.into();
        assert_eq!(num, 6);

        // Test Case 5: Larger array
        let bit_array =
            BitArray::new([Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::One]); // Binary: 101011
        let num: usize = bit_array.into();
        assert_eq!(num, 43);
    }
}
