static ALPHABET: &str = r"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

pub fn b85_encode(bytes: &[u8]) -> String {
    let mut encoded = String::new();

    for e in bytes.chunks(4) {
        let mut b = [0u8; 4];
        b[..(e.len())].copy_from_slice(e);

        let mut x = u32::from_be_bytes(b);
        let mut s = String::new();
        for _ in 0..5 {
            s.push(ALPHABET.chars().nth((x % 85) as usize).unwrap());
            x /= 85;
        }

        encoded += &s.chars().rev().collect::<String>()[..(e.len() + 1)];
    }
    encoded
}

pub fn b85_decode(encoded: &str) -> Vec<u8> {
    let encoded = encoded.chars().collect::<Vec<char>>();
    let mut bytes = Vec::new();

    for e in encoded.chunks(5) {
        let mut x = 0u32;
        for c in e {
            x = x * 85 + ALPHABET.find(*c).unwrap() as u32;
        }

        if e.len() < 5 {
            for _ in 0..=(e.len() % 5) {
                x = (x as u64 * 85 + 84) as u32;
            }
        }

        bytes.extend_from_slice(&x.to_be_bytes()[..(e.len() - 1)]);
    }
    bytes
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

    test! {
        b85_encode:
            test_encode_1: (b"", ""),
            test_encode_2: (b"f", "W&"),
            test_encode_3: (b"fo", "W^V"),
            test_encode_4: (b"foo", "W^Zo"),
            test_encode_5: (b"foob", "W^Zp|"),
            test_encode_6: (b"fooba", "W^Zp|VE"),
            test_encode_7: (b"foobar", "W^Zp|VR8"),
    }

    test! {
        b85_decode:
            test_decode_1: ("", b""),
            test_decode_2: ("W&", b"f"),
            test_decode_3: ("W^V", b"fo"),
            test_decode_4: ("W^Zo", b"foo"),
            test_decode_5: ("W^Zp|", b"foob"),
            test_decode_6: ("W^Zp|VE", b"fooba"),
            test_decode_7: ("W^Zp|VR8", b"foobar"),
    }
}
