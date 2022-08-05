use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Default, Debug)]
pub struct ArgParser {
    pub args: Vec<Arg>,
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
            name: name,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    fn parse_error(&self, msg: &str) -> ! {
        eprintln!("{}", msg);
        println!("{{ usage/help test here }}");
        println!("{self:?}");
        std::process::exit(1)
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
                let opts = arg.trim_start_matches("--").splitn(2, '=');
                let opt = opts.next()
                let (opt, mut val) = arg.trim_start_matches("--").splitn(2, '=').take(2);
                let mut arg = self
                    .args
                    .iter_mut()
                    .find(|e| e.long == Some(opt))
                    .unwrap_or_else(|| todo!());

                match &mut arg.value {
                    ArgValue::Flag(f) if !*f => *f = true,
                    ArgValue::Counter(c) => *c += 1,
                    ArgValue::Value(v) if v.is_none() => {
                        if let Some(val) = {
                            if opt.contains('=') {
                                let mut a = opt.splitn(2, '=');
                                opt = a.next().unwrap();
                                a.next().map(|e| e.to_string())
                            } else {
                                args.next()
                            }
                        } {
                            if let Some(a) = self.args.iter_mut().find(|e| e.long == Some(opt)) {
                                a.value = ArgValue::Value(Some(val));
                            } else {
                                self.parse_error(&format!("unknown option '{}'", opt))
                            }
                        } else {
                            self.parse_error("no value for option")
                        };
                    }
                    ArgValue::List(v) => {
                        if let Some(val) = {
                            if opt.contains('=') {
                                let mut a = opt.splitn(2, '=');
                                opt = a.next().unwrap();
                                a.next().map(|e| e.to_string())
                            } else {
                                args.next()
                            }
                        } {
                            if let Some(a) = self.args.iter_mut().find(|e| e.long == Some(opt)) {
                                a.value = ArgValue::Value(Some(val));
                            } else {
                                self.parse_error(&format!("unknown option '{}'", opt))
                            }
                        } else {
                            self.parse_error("no value for option")
                        };
                    }
                    // ArgValue::Flag(true) | ArgValue::Value(Some(_)) => {
                    _ => {
                        self.parse_error(&format!("cannot pass '{}' twice", opt))
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
                self.parse_error(&format!("missing required argument '{}'", arg.name))
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
