// TODO: In the real world, these keys are allowed to be much larger than u32

#[derive(Clone, Copy)]
pub struct PublicKey {
    pub main_prime: u64, // n
    pub encryption_factor: u32 // e
}

pub struct PrivateKey {
    pub public_key: PublicKey,
    pub decryption_factor: u32 // d
}