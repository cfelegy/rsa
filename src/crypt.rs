use crate::key;

pub fn decrypt(key: key::PrivateKey, ciphered: u64) -> u64 {
    if let Some(raw) = ciphered.checked_pow(key.decryption_factor) {
        let message = raw % key.public_key.main_prime;
        message
    } else {
        panic!("uncipher overflowed");
    }
}

pub fn encrypt(key: key::PublicKey, message: u64) -> u64 {
    if let Some(raw) = message.checked_pow(key.encryption_factor) {
        let cipher = raw % key.main_prime;
        cipher
    } else {
        panic!("cipher overflowed");
    }
}