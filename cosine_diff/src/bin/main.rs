use std::io::{self, Write};

use cosine_diff;

fn main() {
    print!("enter string A: ");
    io::stdout().flush().unwrap();
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    print!("enter string B: ");
    io::stdout().flush().unwrap();
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    println!("difference: {}", cosine_diff::str_diff_n(a.trim(), b.trim(), 2))
}
