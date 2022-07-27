use input::{input, input_password};

fn main() -> std::io::Result<()> {
    let input = input!("integers: ", [i8])?;
    println!("{input:?}");
    let input = input_password!("password: ")?;
    println!("{input:?}");

    Ok(())
}
