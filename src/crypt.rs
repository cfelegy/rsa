use crate::key;
use crate::math;

pub fn decrypt(key: key::PrivateKey, ciphered: u64) -> u64 {
    math::modular_exp(ciphered, key.private_exponent, key.modulus)
}

pub fn encrypt(key: key::PublicKey, message: u64) -> u64 {
    math::modular_exp(message, key.public_exponent, key.modulus)
}