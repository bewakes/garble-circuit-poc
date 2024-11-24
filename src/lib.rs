use std::{
    collections::HashMap,
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    iter::successors,
};

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

impl Bit {
    pub fn from_u64(v: u64) -> Self {
        if v == 0 {
            Self::Zero
        } else {
            Self::One
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

const ZERO: Bit = Bit::Zero;
const ONE: Bit = Bit::One;

type Input = (Bit, Bit);

pub trait Gate {
    fn table(&self) -> [(Input, Bit); 4];

    fn as_str(&self) -> String {
        let table = self.table();
        let mut buffer = String::new();
        buffer.push_str(&format!(
            "Truth Table for {}:\n",
            std::any::type_name::<Self>()
        ));
        buffer.push_str(" A  | B  | Output\n");
        buffer.push_str("----|----|-------\n");
        for ((a, b), output) in table {
            buffer.push_str(&format!(" {}  | {}  | {}\n", a, b, output));
        }
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct AndGate;

impl Gate for AndGate {
    fn table(&self) -> [(Input, Bit); 4] {
        [
            ((ZERO, ZERO), ZERO),
            ((ZERO, ONE), ZERO),
            ((ONE, ZERO), ZERO),
            ((ONE, ONE), ONE),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct GarbledTable<I, H, E> {
    pub input_hash_map: HashMap<(I, I), H>,
    pub input_enc_map: HashMap<(I, I), (E, E)>,
    pub hash_out_map: HashMap<H, E>,
}

impl<I: fmt::Display + fmt::Debug, H: fmt::Display + fmt::Debug, E: fmt::Display + fmt::Debug>
    fmt::Display for GarbledTable<I, H, E>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GarbledTable:")?;

        // Format input_hash_map
        writeln!(f, "\nInput-Hash Map:")?;
        for ((input1, input2), hash) in &self.input_hash_map {
            writeln!(f, "({:?}, {:?}) -> {:?}", input1, input2, hash)?;
        }

        // Format input_enc_map
        writeln!(f, "\nInput-Enc Map:")?;
        for ((input1, input2), (enc1, enc2)) in &self.input_enc_map {
            writeln!(
                f,
                "({:?}, {:?}) -> ({:?}, {:?})",
                input1, input2, enc1, enc2
            )?;
        }

        // Format hash_out_map
        writeln!(f, "\nHash-Out Map:")?;
        for (hash, enc) in &self.hash_out_map {
            writeln!(f, "{:?} -> {:?}", hash, enc)?;
        }

        Ok(())
    }
}

pub trait Garbled {
    type Secret: Hash + Clone;
    type Hash: Hash + Eq + Clone;
    type SymmetricKey; // for password
    type Encrypted: Hash + Clone;

    fn concat(p1: Self::Encrypted, p2: Self::Encrypted) -> Self::Encrypted;
    fn hash(p: &impl Hash) -> Self::Hash;
    fn encrypt_with(psswd: Self::Secret, output: Bit) -> Self::Encrypted;
    fn decrypt_with(psswd: Self::Secret, value: Self::Encrypted) -> Bit;

    // Generate paswords from secret
    fn gen_pwds<'a>(sec: Self::Secret) -> impl Iterator<Item = Self::Secret>;

    fn compute_garble_table(
        secret: Self::Secret,
        gate: &impl Gate,
    ) -> GarbledTable<Bit, Self::Hash, Self::Encrypted> {
        let pwds: Vec<Self::Secret> = Self::gen_pwds(secret).take(12).collect();
        assert!(pwds.len() == 12);

        let get_table_item = |idx: usize, pair: (Bit, Bit)| {
            let pair_enc = (
                Self::encrypt_with(pwds[idx * 3].clone(), pair.0.clone()),
                Self::encrypt_with(pwds[idx * 3 + 1].clone(), pair.1.clone()),
            );
            let (_, out) = gate
                .table()
                .into_iter()
                .find(|(k, _)| *k == pair)
                .expect("inputs not found in table");
            let out_enc = Self::encrypt_with(pwds[idx * 3 + 2].clone(), out);
            (pair_enc, out_enc)
        };

        let concat_hash = |(p1, p2): (Self::Encrypted, Self::Encrypted)| {
            let c = Self::concat(p1, p2);
            Self::hash(&c)
        };

        let i00 = (Bit::Zero, Bit::Zero);
        let i01 = (Bit::Zero, Bit::One);
        let i10 = (Bit::One, Bit::Zero);
        let i11 = (Bit::One, Bit::One);
        let (i00enc, o00enc) = get_table_item(0, i00.clone());
        let (i01enc, o01enc) = get_table_item(1, i01.clone());
        let (i10enc, o10enc) = get_table_item(2, i10.clone());
        let (i11enc, o11enc) = get_table_item(3, i11.clone());
        let i00h = concat_hash(i00enc.clone());
        let i01h = concat_hash(i01enc.clone());
        let i10h = concat_hash(i10enc.clone());
        let i11h = concat_hash(i11enc.clone());

        GarbledTable {
            input_hash_map: HashMap::from([
                (i00.clone(), i00h.clone()),
                (i01.clone(), i01h.clone()),
                (i10.clone(), i10h.clone()),
                (i11.clone(), i11h.clone()),
            ]),
            input_enc_map: HashMap::from([
                (i00, i00enc),
                (i01, i01enc),
                (i10, i10enc),
                (i11, i11enc),
            ]),
            hash_out_map: HashMap::from([
                (i00h, o00enc),
                (i01h, o01enc),
                (i10h, o10enc),
                (i11h, o11enc),
            ]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GarbledGate<G: Gate> {
    secret: u64,
    gate: G,
}

impl<G: Gate> GarbledGate<G> {
    pub fn new(secret: u64, gate: G) -> Self {
        Self { secret, gate }
    }
}

impl<G: Gate> Garbled for GarbledGate<G> {
    type Secret = u64;
    type Hash = u64;
    type SymmetricKey = u64;
    type Encrypted = u64;

    fn hash(v: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish()
    }
    // A very basic encrypt
    fn encrypt_with(p: u64, pout: Bit) -> u64 {
        p + pout.as_u64()
    }

    // A very basic decrypt
    fn decrypt_with(value: u64, p: u64) -> Bit {
        Bit::from_u64(value - p)
    }

    fn concat(p1: Self::Secret, p2: Self::Secret) -> Self::Secret {
        p1 + p2
    }

    fn gen_pwds<'a>(sec: Self::Secret) -> impl Iterator<Item = Self::Secret> {
        let f = |a: &Self::Secret| Some(a * 11 + 3);
        let start = f(&sec);
        successors(start, f)
    }
}
