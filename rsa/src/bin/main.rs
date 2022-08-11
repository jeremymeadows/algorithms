use rsa::Rsa;

fn main() {
    let keypair = Rsa::new();
    let msg = "Hello, world!";
    println!("{:?}", keypair);
    assert_eq!(String::from_utf8(keypair.decrypt(&keypair.encrypt(msg.as_bytes()))).unwrap(), msg);
}