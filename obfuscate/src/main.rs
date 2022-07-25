use obfuscate::{obfuscate};

#[obfuscate]
static VAL: &str = "secret_data_stuff";

//#[obfuscate]
//static VAL2: u8 = 255;

fn main() {
    println!("{}", VAL);
    //println!("{:?}", VAL2);
}
