// TODO: In the real world, these keys are allowed to be much larger than u32
#![allow(dead_code)]

/// An RFC3447 RSA Public Key
#[derive(Clone, Copy)]
pub struct PublicKey {
    pub modulus: u64, // n
    pub public_exponent: u32 // e
}

/// An RFC3447 RSA Private Key
#[derive(Default)]
pub struct PrivateKey {
    pub version: u8,
    pub modulus: u64, // n
    pub public_exponent: u32, // e
    pub private_exponent: u32, // d
    pub prime_1: u32, // p
    pub prime_2: u32, // q
    pub exponent_1: u32, // d mod (p-1)
    pub exponent_2: u32, // d mod (p-2)
    pub coefficient: u32, // inv(q) mod p
    pub other_prime_infos: Option<Vec<OtherPrimeInfo>> // optional per RFC
}

pub struct OtherPrimeInfo {
    prime: u32, // ri
    exponent: u32, // di
    coefficient: u32 // ti
}