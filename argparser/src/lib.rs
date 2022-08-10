use std::fmt::{self, Display, Formatter};

#[derive(Default, Debug)]
pub struct ArgParser {
    name: &'static str,
    args: Vec<Arg>,
}

#[derive(Default, Debug)]
pub struct Arg {
    name: &'static str,
    description: &'static str,
    long: Option<&'static str>,
    short: Option<char>,
    required: bool,
    value: ArgValue,
}

#[derive(Debug)]
pub enum ArgValue {
    Flag(bool),
    Counter(u8),
    Value(Option<String>),
    List(Vec<String>),
}

impl Default for ArgValue {
    fn default() -> Self {
        Self::Flag(false)
    }
}

impl Display for ArgValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ArgValue::Flag(x) => write!(f, "{}", x),
            ArgValue::Counter(c) => write!(f, "{}", c),
            ArgValue::Value(Some(v)) => write!(f, "{}", v),
            ArgValue::List(v) => write!(f, "{:?}", v),
            ArgValue::Value(None) => write!(f, "None"),
        }
    }
}

impl Arg {
    pub fn new(name: &'static str) -> Self {
        Arg {
            name,
            ..Default::default()
        }
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn value(mut self) -> Self {
        self.value = ArgValue::Value(None);
        self
    }

    pub fn default(mut self, val: ArgValue) -> Self {
        self.value = val;
        self
    }
}

impl ArgParser {
    pub fn new(name: &'static str) -> Self {
        Self { name, ..Default::default() }
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    fn print_error(&self, msg: &str) -> ! {
        eprintln!("{}", msg);
        println!("{}", self.help());
        std::process::exit(1)
    }

    fn help(&self) -> String {
        let mut s = vec!["Usage:".to_string()];

        s.push(format!("    {}", self.name));
        s.push(format!("\nOptions:"));

        s.join("\n")
    }

    pub fn parse(mut self) -> Self {
        for arg in self.args.iter_mut() {
            if arg.required {
                arg.value = ArgValue::Value(None);
            }
            if arg.long.is_none() && arg.short.is_none() {
                arg.value = ArgValue::Value(None);
            }
        }

        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let mut opts = arg.trim_start_matches("--").splitn(2, '=');
                let opt = opts.next().unwrap();

                let mut arg = self
                    .args
                    .iter_mut()
                    .find(|e| e.long == Some(opt))
                    .unwrap_or_else(|| panic!("unknown option '{}'", opt));

                match &mut arg.value {
                    ArgValue::Flag(f) if !*f => *f = true,
                    ArgValue::Counter(c) => *c += 1,
                    ArgValue::Value(v) if v.is_none() => {
                        if let Some(val) = {
                            opts.next().map(|e| e.to_string()).or_else(|| args.next())
                        } {
                            arg.value = ArgValue::Value(Some(val));
                        } else {
                            self.print_error("no value for option")
                        };
                    }
                    ArgValue::List(v) => {
                        if let Some(val) = {
                            opts.next().map(|e| e.to_string()).or_else(|| args.next())
                        } {
                            v.push(val);
                        } else {
                            self.print_error("no value for option")
                        };
                    }
                    _ => {
                        self.print_error(&format!("cannot pass '{}' twice", opt))
                    }
                }
            } else if arg.starts_with("-") {
            } else {
                if let Some(a) = self.args.iter_mut().find(|e| {
                    matches!(e.value, ArgValue::Value(None))
                        && e.long.is_none()
                        && e.short.is_none()
                }) {
                    a.value = ArgValue::Value(Some(arg));
                }
            }
        }

        for arg in self.args.iter() {
            if let ArgValue::Value(None) = arg.value {
                self.print_error(&format!("missing required argument '{}'", arg.name))
            }
        }
        self
    }

    pub fn get(&self, name: &str) -> &ArgValue {
        &self.get_opt(name).unwrap()
    }

    pub fn get_opt(&self, name: &str) -> Option<&ArgValue> {
        self.args
            .iter()
            .find(|arg| arg.name == name)
            .map(|e| &e.value)
    }
}
