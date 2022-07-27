#[macro_export]
macro_rules! input {
    ($($prompt:literal)?) => {{
        use std::io::{self, Write};

        $(
            print!($prompt);
            io::stdout().flush()?;
        )?

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok::<String, io::Error>(line.trim().to_string())
    }};
    ($($prompt:literal,)? [$t:ty]) => {
        input!($($prompt)?).and_then(|input| {
            use std::io::{Error, ErrorKind};

            let v = input
                .split_whitespace()
                .map(|e| e
                    .parse::<$t>()
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))
                );

            if let Some(Err(err)) = v.clone().find(|e| e.is_err()) {
                Err(err)
            } else {
                Ok(v.map(|e| e.unwrap()).collect::<Vec<$t>>())
            }
        })
    };
    ($($prompt:literal,)? $t:ty) => {
        input!($($prompt)?).and_then(|input| {
            use std::io::{Error, ErrorKind};

            input
                .parse::<$t>()
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))
        })
    };
    ($($prompt:literal,)? $($t:ty),+) => {
        input!($($prompt)?).and_then(|input| {
            use std::io::{Error, ErrorKind};

            let mut toks = input.split_whitespace();
            let mut err = None;

            let tup = ($(
                if let Some(e) = toks.next() {
                    match e.parse::<$t>() {
                        Ok(x) => x,
                        Err(e) => {
                            err = Some(e.to_string());
                            <$t>::default()
                        },
                    }
                } else {
                    err = Some("missing input argument(s)".to_string());
                    <$t>::default()
                }
            ),+);

            if toks.next().is_none() && err.is_none() {
                Ok(tup)
            } else {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    err.unwrap_or("recieved more input arguments than expected".to_string())
                ))
            }
        })
    };
}

#[macro_export]
macro_rules! input_password {
    ($prompt:literal $(, $tok:tt),*) => {{
        use libc::{termios, ECHO, ECHONL, STDIN_FILENO, TCSANOW};

        let mut term = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        unsafe {
            libc::tcgetattr(STDIN_FILENO, &mut term);
        }

        {
            let mut term = term.clone();
            term.c_lflag &= !ECHO; // don't echo input characters
            term.c_lflag |= ECHONL; // do echo the trailing newline

            unsafe {
                libc::tcsetattr(STDIN_FILENO, TCSANOW, &term);
            }
        }
        let input = input!($prompt $(,$tok)*);

        unsafe {
            libc::tcsetattr(STDIN_FILENO, TCSANOW, &term);
        }
        input
    }};
}
