use std::hash::Hash;

pub trait EncryptionScheme {
    type Key: Hash;
    type Value;
    type EncryptedValue;

    fn encrypt(&self, v: Self::Value) -> Self::EncryptedValue;
    fn decrypt(&self, e: Self::EncryptedValue) -> Self::Value;
}

pub struct SimpleEncryptionScheme(pub u64);

impl EncryptionScheme for SimpleEncryptionScheme {
    type Key = u64;
    type Value = u64;
    type EncryptedValue = u64;

    fn encrypt(&self, v: Self::Value) -> Self::EncryptedValue {
        self.0 + v
    }

    fn decrypt(&self, e: Self::EncryptedValue) -> Self::Value {
        e - self.0
    }
}
