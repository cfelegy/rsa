use crate::key;

pub fn decrypt(key: key::PrivateKey, ciphered: u64) -> u64 {
    if let Some(raw) = ciphered.checked_pow(key.private_exponent) {
        let message = raw % key.modulus;
        message
    } else {
        panic!("uncipher overflowed");
    }
}

pub fn encrypt(key: key::PublicKey, message: u64) -> u64 {
    if let Some(raw) = message.checked_pow(key.public_exponent) {
        let cipher = raw % key.modulus;
        cipher
    } else {
        panic!("cipher overflowed");
    }
}