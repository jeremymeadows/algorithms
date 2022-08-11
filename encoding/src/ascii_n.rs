//! Implementation of the SHA1-HMAC Time-Based One-Time Password Algorithm.
//!
//! Passes the IETF test vectors from [RFC 6238](https://www.rfc-editor.org/rfc/rfc6238).

macro_rules! ascii_encode {
    ($($name:ident: $base:literal,)*) => {
        $(
            pub fn $name(bytes: &[u8]) -> String {
                let mut encoded = String::new();

                for e in bytes.chunks(4) {
                    let mut b = [0u8; 4];
                    b[..(e.len())].copy_from_slice(e);

                    let mut x = u32::from_be_bytes(b);
                    let mut s = String::new();
                    for _ in 0..5 {
                        s.push(((x % $base) + 33) as u8 as char);
                        x /= $base;
                    }

                    encoded += &s.chars().rev().collect::<String>()[..(e.len() + 1)];
                }
                encoded
            }
        )*
    }
}

macro_rules! ascii_decode {
    ($($name:ident: $base:literal,)*) => {
        $(
            pub fn $name(encoded: &str) -> Vec<u8> {
                let encoded = encoded.chars().collect::<Vec<char>>();
                let mut bytes = Vec::new();

                for e in encoded.chunks(5) {
                    let mut x = 0u32;
                    for c in e {
                        x = x * $base + *c as u32 - 33;
                    }

                    if e.len() < 5 {
                        for _ in 0..=(e.len() % 5) {
                            x = x * $base + $base - 1;
                        }
                    }

                    bytes.extend_from_slice(&x.to_be_bytes()[..(e.len() - 1)]);
                }
                bytes
            }
        )*
    }
}

ascii_encode!(
    a85_encode: 85,
);

ascii_decode!(
    a85_decode: 85,
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($f:ident: $($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $values;
                    assert_eq!($f(input), expected);
                }
            )*
        }
    }

    test! {
        a85_encode:
            test_encode_1: (b"", ""),
            test_encode_2: (b"f", "Ac"),
            test_encode_3: (b"fo", "Ao@"),
            test_encode_4: (b"foo", "AoDS"),
            test_encode_5: (b"foob", "AoDTs"),
            test_encode_6: (b"fooba", "AoDTs@/"),
            test_encode_7: (b"foobar", "AoDTs@<"),
    }

    test! {
        a85_decode:
            test_decode_1: ("", b""),
            test_decode_2: ("Ac", b"f"),
            test_decode_3: ("Ao@", b"fo"),
            test_decode_4: ("AoDS", b"foo"),
            test_decode_5: ("AoDTs", b"foob"),
            test_decode_6: ("AoDTs@/", b"fooba"),
            test_decode_7: ("AoDTs@<", b"foobar"),
    }
}