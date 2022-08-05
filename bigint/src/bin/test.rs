use bigint::BigInt;
use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();

    let a = BigInt::from_str(input.next().unwrap()).unwrap();
    let b = BigInt::from_str(input.next().unwrap()).unwrap();
    let c = BigInt::from_str(input.next().unwrap()).unwrap();

    println!("{:x}", &a + &b);
    println!("{:x}", &a - &b);
    println!("{:x}", &a * &b);
    println!("{:x}", &a / &b);
    // println!("{:x}", &a % &b);
    println!("{:x}", &a.modulo(&b));
    // println!("{:x}", &a.mod_pow(&c, &b));
}
