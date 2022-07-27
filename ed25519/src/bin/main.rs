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
    let public = ed25519::secret_to_public(&secret);

    print!("enter message: ");
    io::stdout().flush().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let message = input.trim().as_bytes();
    let sig = ed25519::sign(&secret, &message);

    println!(
        "
secret key: {}
public key: {}

signature: {}
verified: {}",
        encoding::b16_encode(&*secret),
        encoding::b16_encode(&public),
        encoding::b64_encode(&sig),
        ed25519::verify(&public, &message, &sig)
    )
}
