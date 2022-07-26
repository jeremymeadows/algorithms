//! Library to manage encoding/decoding base64/base32/base16 data.
//!
//! Passes the IETF test vectors from [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648).

const B64_ALPHABET_URL_SAFE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const B64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const B64_PAD: u8 = 4;

const B32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const B32_PAD: u8 = 8;

const B16_ALPHABET: &str = "0123456789ABCDEF";
const B16_PAD: u8 = 1;

// Generates an `encode` method with the given `name` for the specified `alphabet`, padding the
// result out to `pad` bytes for power-of-two bases.
macro_rules! base_encode {
    ($($name:ident: $alph:expr, $pad:expr,)*) => {
        $(
            pub fn $name(bytes: &[u8]) -> String {
                let bit_count = ($alph.len() as f32).log2() as usize;

                let mut bstr = String::new();
                let mut encoded = String::new();

                for b in bytes {
                    bstr.push_str(&format!("{:08b}", b));
                }
                while bstr.len() % bit_count != 0 {
                    bstr.push('0');
                }

                for c in bstr.chars().collect::<Vec<char>>().chunks(bit_count) {
                    encoded.push(
                        $alph
                            .chars()
                            .nth(
                                u8::from_str_radix(&c.iter().collect::<String>(), 2)
                                    .unwrap()
                                    .into()
                            )
                            .unwrap()
                    );
                }

                while encoded.len() % $pad as usize != 0 {
                    encoded.push('=');
                }
                encoded
            }
        )*
    }
}

// Generates a `decode` method with the given `name` for the specified `alphabet`, expecting the
// input to be padded to `pad` bytes for power-of-two bases.
macro_rules! base_decode {
    ($($name:ident: $alph:expr, $pad:expr,)*) => {
        $(
            pub fn $name(encoded: &str) -> Vec<u8> {
                let bit_count = ($alph.len() as f32).log2() as usize;

                let mut bstr = String::new();
                let mut bytes = Vec::new();

                for c in encoded.trim_end_matches('=').chars() {
                    let n = $alph.find(c).unwrap();
                    bstr.push_str(&format!("{n:0bit_count$b}"));
                }
                while bstr.len() % 8 != 0 {
                    bstr.push('0');
                }

                for b in bstr.chars().collect::<Vec<char>>().chunks(8) {
                    match u8::from_str_radix(&b.iter().collect::<String>(), 2) {
                        Ok(b) => bytes.push(b),
                        Err(_) => panic!("invalid encoded string"),
                    }
                }

                if Some(&0) == bytes.iter().last() {
                    bytes.pop().unwrap();
                }
                bytes
            }
        )*
    };
}

base_encode! {
    b64_encode_urlsafe: B64_ALPHABET_URL_SAFE, B64_PAD,
    b64_encode: B64_ALPHABET, B64_PAD,
    b32_encode: B32_ALPHABET, B32_PAD,
    b16_encode: B16_ALPHABET, B16_PAD,
}

base_decode! {
    b64_decode_urlsafe: B64_ALPHABET_URL_SAFE, B64_PAD,
    b64_decode: B64_ALPHABET, B64_PAD,
    b32_decode: B32_ALPHABET, B32_PAD,
    b16_decode: B16_ALPHABET, B16_PAD,
}

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

    mod base64 {
        use super::{b64_decode, b64_encode};

        test! {
            b64_encode:
                test_encode_1: (b"", ""),
                test_encode_2: (b"f", "Zg=="),
                test_encode_3: (b"fo", "Zm8="),
                test_encode_4: (b"foo", "Zm9v"),
                test_encode_5: (b"foob", "Zm9vYg=="),
                test_encode_6: (b"fooba", "Zm9vYmE="),
                test_encode_7: (b"foobar", "Zm9vYmFy"),
        }

        test! {
            b64_decode:
                test_decode_1: ("", b""),
                test_decode_2: ("Zg==", b"f"),
                test_decode_3: ("Zm8=", b"fo"),
                test_decode_4: ("Zm9v", b"foo"),
                test_decode_5: ("Zm9vYg==", b"foob"),
                test_decode_6: ("Zm9vYmE=", b"fooba"),
                test_decode_7: ("Zm9vYmFy", b"foobar"),
        }
    }

    mod base32 {
        use super::{b32_decode, b32_encode};

        test! {
            b32_encode:
                test_encode_1: (b"", ""),
                test_encode_2: (b"f", "MY======"),
                test_encode_3: (b"fo", "MZXQ===="),
                test_encode_4: (b"foo", "MZXW6==="),
                test_encode_5: (b"foob", "MZXW6YQ="),
                test_encode_6: (b"fooba", "MZXW6YTB"),
                test_encode_7: (b"foobar", "MZXW6YTBOI======"),
        }

        test! {
            b32_decode:
                test_decode_1: ("", b""),
                test_decode_2: ("MY======", b"f"),
                test_decode_3: ("MZXQ====", b"fo"),
                test_decode_4: ("MZXW6===", b"foo"),
                test_decode_5: ("MZXW6YQ=", b"foob"),
                test_decode_6: ("MZXW6YTB", b"fooba"),
                test_decode_7: ("MZXW6YTBOI======", b"foobar"),
        }
    }

    mod base16 {
        use super::{b16_decode, b16_encode};

        test! {
            b16_encode:
                test_encode_1: (b"", ""),
                test_encode_2: (b"f", "66"),
                test_encode_3: (b"fo", "666F"),
                test_encode_4: (b"foo", "666F6F"),
                test_encode_5: (b"foob", "666F6F62"),
                test_encode_6: (b"fooba", "666F6F6261"),
                test_encode_7: (b"foobar", "666F6F626172"),
        }

        test! {
            b16_decode:
                test_decode_1: ("", b""),
                test_decode_2: ("66", b"f"),
                test_decode_3: ("666F", b"fo"),
                test_decode_4: ("666F6F", b"foo"),
                test_decode_5: ("666F6F62", b"foob"),
                test_decode_6: ("666F6F6261", b"fooba"),
                test_decode_7: ("666F6F626172", b"foobar"),
        }
    }

    // #[test]
    // fn test_encode_b64_trailing_0() {
    //     assert_eq!(&b64_encode(b"Hello!\x00"), "SGVsbG8hAA==");
    // }

    // #[test]
    // fn test_decode_b64_trailing_0() {
    //     assert_eq!(b64_decode("SGVsbG8hAA=="), b"Hello!\x00");
    // }

    // #[test]
    // fn test_encode_b32_trailing_0() {
    //     assert_eq!(&b32_encode(b"Hello!\x00"), "JBSWY3DPEEAA====");
    // }

    // #[test]
    // fn test_decode_b32_trailing_0() {
    //     assert_eq!(b32_decode("JBSWY3DPEEAA===="), b"Hello!\x00");
    // }
}
