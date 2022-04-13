mod crypt;
mod key;

fn main() {
    let pubkey = key::PublicKey {
        modulus: 33,
        public_exponent: 7
    };
    let message = 2;
    let encrypted = crypt::encrypt(pubkey, message);
    println!("{} encrypted to {}", message, encrypted);

    let private_key = key::PrivateKey {
        modulus: pubkey.modulus,
        public_exponent: pubkey.public_exponent,
        private_exponent: 3,
        ..Default::default()
    };
    let decrypted = crypt::decrypt(private_key, encrypted);
    println!("{} decrypted to {}", encrypted, decrypted);

    assert_eq!(message, decrypted);
}
