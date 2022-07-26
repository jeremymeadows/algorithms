use std::io::{self, Write};

use ed25519;
use encoding;
use sha::{sha256::Sha256, Sha};

fn main() {
    let mut input;

    print!("enter password: ");
    io::stdout().flush().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let secret = Sha256::hash(input.trim().as_bytes());
    let public = ed25519::secret_to_public(&secret).1;

    print!("enter message: ");
    io::stdout().flush().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let message = input.trim().as_bytes();
    let sig = ed25519::sign(&secret, &message);

    println!("");
    println!("secret key: {}", encoding::b16_encode(&*secret));
    println!("public key: {}", encoding::b16_encode(&public));

    println!("");
    println!("signature: {}", encoding::b16_encode(&sig));
    println!("verified: {}", ed25519::verify(&public, &message, &sig))
}
