use proc_macro::{TokenStream, TokenTree::Literal};
use std::str::FromStr;

#[proc_macro_attribute]
pub fn obfuscate(_: TokenStream, expr: TokenStream) -> TokenStream {
    let mut e = expr.to_string();

    for tok in expr.clone() {
        let type_start = e.find(':').unwrap();
        let type_end = e.find('=').unwrap();

        let ty = &e[(type_start + 1)..type_end].trim().to_owned();
        let owned_ty = if ty.to_lowercase().contains("str") {
            "String"
        } else {
            ty
        };

        if let Literal(lit) = tok {
            let mut s = String::new();

            for (i, e) in lit
                .to_string()
                .trim_matches('"')
                .as_bytes()
                .iter()
                .enumerate()
            {
                s = format!("{}{},", s, i as u8 ^ !e);
            }

            e = e.replace(ty, &format!("obfuscate::Obfuscated<{owned_ty}>"));
            e = e.replace(&lit.to_string(), &format!("obfuscate::Obfuscated::<{owned_ty}>::new(||
                String::from_utf8(
                    [{s}]
                        .iter()
                        .enumerate()
                        .map(|(i, e)| i as u8 ^ !e as u8)
                        .collect::<Vec<u8>>()
                )
                .unwrap()
            )"));
        }
    }

    TokenStream::from_str(&e).unwrap()
}
