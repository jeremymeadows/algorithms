//! Macros to read and parse user input from standard input, with the ability to hide input
//! characters, or read before a newline is found.

/// Reads and parses standard input into a tuple, array, or a type which implements [`FromStr`].
/// The prompt is optional, and will print before blocking on user input if it is provided.
/// 
/// ```
/// use input::input;
/// 
/// let line: Result<String, _> = input!("enter some text then press `return`: ");
/// 
/// let int: Result<i32, _> = input!("enter an integer: ", i32);
/// 
/// let tup: Result<(String, f32, bool), _> =
///     input!("enter a string, a float, then true/false: ", (String, f32, bool));
/// 
/// let arr: Result<Vec<i32>, _> = input!("enter a lot of integers: ", [i32]);
/// ```
/// 
/// [`FromStr`]: std::str::FromStr
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
    ($($prompt:literal,)? ($($t:ty),+)) => {
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
}

/// Performes the same actions as [`input!`], but does not echo any keypresses to standard
/// out, and requires a prompt.
/// 
/// ```
/// use input::input_password;
/// 
/// let pass: Result<String, _> = input_password!("enter your password: ");
/// let num: Result<u8, _> = input_password!("now enter your secret number: ", u8);
/// ```
/// 
/// [`input!`]: crate::input
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
            #[cfg(not(any(
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "mips",
                target_arch = "mips64")))]
            c_ispeed: 0,
            #[cfg(not(any(
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "mips",
                target_arch = "mips64")))]
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

/// Gets a single character from standard input without echoing any input or waiting for
/// an end-of-line character. Takes an optional argument to print a prompt to standard out.
/// 
/// ```
/// use input::read_char;
/// 
/// _ = read_char!("press any key to continue...");
/// println!("done!");
/// ```
#[macro_export]
macro_rules! read_char {
    ($($prompt:literal)?) => {{
        use std::io::{self, Write};
        use libc::{termios, ECHO, ECHONL, ICANON, STDIN_FILENO, TCSANOW};

        $(
            print!($prompt);
            io::stdout().flush()?;
        )?

        let mut term = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            #[cfg(not(any(
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "mips",
                target_arch = "mips64")))]
            c_ispeed: 0,
            #[cfg(not(any(
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "mips",
                target_arch = "mips64")))]
            c_ospeed: 0,
        };
        unsafe {
            libc::tcgetattr(STDIN_FILENO, &mut term);
        }

        {
            let mut term = term.clone();
            term.c_lflag &= !ICANON; // leave canonical mode, which provides input before EOL
            term.c_lflag &= !ECHO; // don't echo input characters
            term.c_lflag |= ECHONL; // do echo the trailing newline

            unsafe {
                libc::tcsetattr(STDIN_FILENO, TCSANOW, &term);
            }
        }
        let c = unsafe { libc::getchar() } as u8;

        unsafe {
            libc::tcsetattr(STDIN_FILENO, TCSANOW, &term);
        }
        c as char
    }};
}
