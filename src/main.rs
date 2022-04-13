mod crypt;
mod key;

fn main() {
    let pubkey = key::PublicKey {
        main_prime: 33,
        encryption_factor: 7
    };
    let message = 2;
    let encrypted = crypt::encrypt(pubkey, message);
    println!("{} encrypted to {}", message, encrypted);

    let private_key = key::PrivateKey {
        public_key: pubkey.clone(),
        decryption_factor: 3
    };
    let decrypted = crypt::decrypt(private_key, encrypted);
    println!("{} decrypted to {}", encrypted, decrypted);

    assert_eq!(message, decrypted);
}
