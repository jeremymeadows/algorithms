use argparser::{Arg, ArgParser, ArgValue};

fn main() {
    let args = ArgParser::new("test app")
        .arg(Arg::new("mode").description("read or write"))
        .arg(
            Arg::new("file")
                .description("file to read")
                .long("file")
                .short('f')
                .value(),
        )
        .arg(Arg::new("type").description("file type").short('t').value())
        .parse();

    println!("mode is {}", args.get("mode"));
    println!("file is {}", args.get("file"));

    if let Some(ty) = args.get_opt("type") {
        println!("type is {ty}");
    }
}
