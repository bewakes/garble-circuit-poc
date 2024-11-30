use std::{fmt, ops::Deref};

use crate::bit::Bit;

pub struct BitArray<const I: usize> {
    inner: [Bit; I],
}

impl<const I: usize> BitArray<I> {
    pub fn new(inner: [Bit; I]) -> Self {
        Self { inner }
    }
}

impl<const I: usize> Deref for BitArray<I> {
    type Target = [Bit; I];
    fn deref(&self) -> &Self::Target {
        &self.inner
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

pub type Table<const I: usize> = [bool; 1 << I];

#[derive(Clone, Debug)]
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
    pub fn from_table(mut raw_table: [([Bit; I], Bit); 1 << I]) -> Self {
        raw_table.sort_by(|a, b| a.0.cmp(&b.0));
        let mut table = [false; 1 << I];
        table.copy_from_slice(
            &raw_table
                .iter()
                .map(|&(_, bit)| bool::from(bit))
                .collect::<Vec<_>>(),
        );
        Self { table }
    }

    pub fn evaluate(&self, input: &[Bit; I]) -> Bit {
        let index: usize = BitArray::new(*input).into();
        let val = self.table[index];
        val.into()
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
        for (i, output) in self.table.iter().enumerate() {
            let bitarr: BitArray<I> = i.into();
            let input_str = bitarr
                .inner
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
    table: [false, false, false, true],
};

pub const ORGATE: Gate<2> = Gate {
    table: [false, true, true, true],
};

pub const XORGATE: Gate<2> = Gate {
    table: [false, true, true, false],
};

pub const NANDGATE: Gate<2> = Gate {
    table: [true, true, true, false],
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

    #[test]
    fn to_and_from() {
        const BITS: usize = 15;
        for i in 0usize..1 << BITS {
            let barr: BitArray<BITS> = i.into();
            let res: usize = barr.into();
            assert_eq!(i, res);
        }
    }
}
