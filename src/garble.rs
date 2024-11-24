use std::{
    collections::HashMap,
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    iter::successors,
};

use crate::gate::{Bit, Gate};

impl<H: Hash + fmt::Display + fmt::Debug, E: fmt::Display + fmt::Debug> fmt::Display
    for GarbledTable<H, E>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format input_enc_map
        writeln!(f, "Input-Encoding by symmetric encryption :")?;
        for ((input1, input2), (enc1, enc2)) in &self.input_enc_map {
            writeln!(
                f,
                "({:?}, {:?}) -> ({:?}, {:?})",
                input1, input2, enc1, enc2
            )?;
        }
        // Format input_hash_map
        writeln!(f, "\nHashes for corresponding inputs:")?;
        for ((input1, input2), hash) in &self.input_hash_map {
            writeln!(f, "({:?}, {:?}) -> {:?}", input1, input2, hash)?;
        }

        // Format hash_out_map
        writeln!(f, "\nEncrypted output for each input hash")?;
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

    fn master_secret(&self) -> Self::Secret;
    fn gate(&self) -> &impl Gate;

    fn concat(p1: Self::Encrypted, p2: Self::Encrypted) -> Self::Encrypted;
    fn hash(p: &impl Hash) -> Self::Hash;
    fn encrypt_with(psswd: Self::Secret, output: Bit) -> Self::Encrypted;
    fn decrypt_with(psswd: Self::Secret, value: Self::Encrypted) -> Bit;

    // Generate secrets from secret
    fn gen_pwds<'a>(sec: Self::Secret) -> impl Iterator<Item = Self::Secret>;

    fn compute_garble_table(&self) -> GarbledTable<Self::Hash, Self::Encrypted> {
        let pwds: Vec<Self::Secret> = Self::gen_pwds(self.master_secret()).take(12).collect();
        assert!(pwds.len() == 12);

        let concat_hash = |(p1, p2): (Self::Encrypted, Self::Encrypted)| {
            let c = Self::concat(p1, p2);
            Self::hash(&c)
        };
        let table = self.gate().table();
        let mut input_hash_map = HashMap::new();
        let mut input_enc_map = HashMap::new();
        let mut hash_out_map = HashMap::new();

        for (i, (inp, out)) in table.iter().enumerate() {
            // Encrypt inputs and output
            let encrypted_inputs = (
                Self::encrypt_with(pwds[i * 3].clone(), inp.0.clone()),
                Self::encrypt_with(pwds[i * 3 + 1].clone(), inp.1.clone()),
            );
            // let encrypted_output = Self::encrypt_with(pwds[i * 3 + 2].clone(), out.clone());

            // Compute hash for encrypted inputs
            let input_hash = concat_hash(encrypted_inputs.clone());

            // Populate maps
            input_hash_map.insert(inp.clone(), input_hash.clone());
            input_enc_map.insert(inp.clone(), encrypted_inputs);
            hash_out_map.insert(input_hash, out.clone());
        }

        GarbledTable {
            input_hash_map,
            input_enc_map,
            hash_out_map,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleGarbledGate<G: Gate> {
    /// master secret
    master_secret: u64,
    /// The gate that is garbled
    gate: G,
}

impl<G: Gate> SimpleGarbledGate<G> {
    pub fn new(master_secret: u64, gate: G) -> Self {
        Self {
            master_secret,
            gate,
        }
    }
}

impl<G: Gate> Garbled for SimpleGarbledGate<G> {
    type Secret = u64;
    type Hash = u64;
    type SymmetricKey = u64;
    type Encrypted = u64;

    fn master_secret(&self) -> Self::Secret {
        self.master_secret
    }

    fn gate(&self) -> &impl Gate {
        &self.gate
    }

    fn hash(v: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish()
    }
    // A very basic encrypt
    fn encrypt_with(p: u64, pout: Bit) -> u64 {
        let pout: u64 = pout.into();
        p + pout
    }

    // A very basic decrypt
    fn decrypt_with(value: u64, p: u64) -> Bit {
        (value - p).into()
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

#[derive(Clone, Debug)]
pub struct GarbledTable<H: Hash, E> {
    pub input_hash_map: HashMap<(Bit, Bit), H>,
    pub input_enc_map: HashMap<(Bit, Bit), (E, E)>,
    pub hash_out_map: HashMap<H, Bit>,
}

impl<H: Clone + Eq + Hash, E: Clone> GarbledTable<H, E> {
    /// Returns the sub table whose first input is inp
    pub fn get_table_for_input(&self, inp: Bit) -> HashMap<H, Bit> {
        let tb: Vec<_> = self
            .input_hash_map
            .iter()
            .filter_map(|(i, h)| {
                if i.0 == inp {
                    self.hash_out_map.get(h).map(|x| (h.clone(), x.clone()))
                } else {
                    None
                }
            })
            .collect();
        HashMap::from_iter(tb)
    }
}
