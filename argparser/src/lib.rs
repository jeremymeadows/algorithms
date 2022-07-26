use std::collections::HashMap;

#[derive(Default)]
pub struct ArgParser {
    args: Vec<Arg>,
    pub arg_values: HashMap<String, ArgType>,
}

#[derive(Default)]
pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    //help: Option<String>,
    pub required: bool,
    pub multiple: bool,
    pub value: bool,
}

#[derive(Debug)]
pub enum ArgType {
    Flag,
    Value(String),
}

impl ArgParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn parse(mut self) -> Self {
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let arg = arg
                    .trim_start_matches("--")
                    .splitn(2, "=")
                    .collect::<Vec<_>>();
                let arg_meta = match self.args.iter().find(|a| a.long == Some(arg[0])) {
                    Some(a) => a,
                    None => panic!("invalid argument: {}", arg[0]),
                };

                self.arg_values.insert(
                    arg_meta.name.to_string(),
                    if arg_meta.value {
                        if arg.len() == 2 {
                            ArgType::Value(arg[1].to_string())
                        } else {
                            match args.next() {
                                Some(v) if !v.starts_with("-") => {
                                    
                                }
                                _ => {}
                            }
                            ArgType::Value(args.next().expect(&format!("{} requires a value", arg[0])))
                        }
                    } else {
                        ArgType::Flag
                    },
                );
            } else if arg.starts_with("-") {
                let _arg = arg
                    .trim_start_matches("-")
                    .splitn(2, "=")
                    .collect::<Vec<_>>();
            }
        }
        // for arg in &self.args {
        //     let aa = args.iter().filter(|(_, a)|
        //         arg.short.is_some() && a == &format!("-{}", arg.short.unwrap()) ||
        //         arg.long.is_some() && a == &format!("--{}", arg.long.unwrap())
        //     ).collect::<Vec<_>>();

        //     if arg.required && aa.len() == 0 {
        //         panic!("Argument {} is required", arg.name);
        //     }
        //     // else if !arg.multiple && aa.count() > 1 {
        //     //     panic!("Argument {} is not multiple", arg.name);
        //     // }
        //     match aa.first() {
        //         Some(a) => {
        //             if arg.value {
        //                 self.arg_values.insert(arg.name.to_string(), ArgType::Value(args.iter().nth(a.0 + 1).unwrap().1.clone()));
        //             } else {
        //                 self.arg_values.insert(arg.name.to_string(), ArgType::Flag(true));
        //             }
        //         }
        //         None => {
        //             if arg.required {
        //                 println!("{} is required", arg.name);
        //                 std::process::exit(1);
        //             }
        //         }
        //     }
        // }
        self
    }

    pub fn get(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(|arg| arg.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let args = ArgParser::new()
            .arg(Arg {
                name: "no capture",
                long: Some("nocapture"),
                ..Default::default()
            })
            .arg(Arg {
                name: "has value",
                long: Some("value"),
                ..Default::default()
            })
            .parse();
        println!("{:?}", std::env::args().collect::<Vec<String>>());
        match args.get("no capture") {
            Some(_) => {
                println!("found no capture");
            }
            None => {}
        }
        println!("{:?}", args.arg_values);
    }
}
