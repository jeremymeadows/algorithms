use input::{input, input_password, read_char};

fn main() -> std::io::Result<()> {
    println!("{:?}", input!("integers: ", [i32])?);
    println!("{:?}", input_password!("password: ")?);
    read_char!("press any key to continue...\n");

    Ok(())
}
