use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Error, Expr, ExprArray, ExprLit, ExprRepeat, Lit, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};
use tiny_keccak::{Hasher, Keccak};

struct KeccakInput {
    bytes: Vec<u8>,
}

fn parse_u8_expr(expr: &Expr) -> Result<u8> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => lit_int.base10_parse::<u8>(),
        _ => Err(Error::new(expr.span(), "expected u8 integer literal")),
    }
}

fn extract_bytes(expr: &Expr) -> Result<Vec<u8>> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => {
            let val = lit_int.base10_parse::<u8>()?;
            Ok(vec![val])
        }
        Expr::Lit(ExprLit {
            lit: Lit::ByteStr(lit_bs),
            ..
        }) => Ok(lit_bs.value()),
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_s),
            ..
        }) => {
            let s = lit_s.value();
            if let Some(hex_str) = s.strip_prefix("0x") {
                hex::decode(hex_str)
                    .map_err(|e| Error::new(lit_s.span(), format!("invalid hex string: {e}")))
            } else {
                Ok(s.into_bytes())
            }
        }
        Expr::Array(ExprArray { elems, .. }) => {
            let mut res = Vec::with_capacity(elems.len());
            for elem in elems {
                res.push(parse_u8_expr(elem)?);
            }
            Ok(res)
        }
        Expr::Repeat(ExprRepeat {
            expr: elem, len, ..
        }) => {
            let byte_val = parse_u8_expr(elem)?;
            let count = match len.as_ref() {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(lit_int),
                    ..
                }) => lit_int.base10_parse::<usize>()?,
                _ => {
                    return Err(Error::new(
                        len.span(),
                        "expected integer length in array repeat",
                    ));
                }
            };
            Ok(vec![byte_val; count])
        }
        _ => Err(Error::new(
            expr.span(),
            "expected integer literal, string literal, byte array [u8; N], or byte slice",
        )),
    }
}

impl Parse for KeccakInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut bytes = Vec::new();
        for expr in punctuated {
            let mut b = extract_bytes(&expr)?;
            bytes.append(&mut b);
        }
        Ok(Self { bytes })
    }
}

pub fn expand_keccak256(input: TokenStream2) -> Result<TokenStream2> {
    let parsed: KeccakInput = syn::parse2(input)?;
    let mut hasher = Keccak::v256();
    hasher.update(&parsed.bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    let byte_literals = output.iter().map(|b| quote! { #b });
    Ok(quote! {
        [#(#byte_literals),*]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_keccak256() {
        let input = quote!(2u8, [0u8; 20]);
        let expanded = expand_keccak256(input).unwrap();
        let expected = quote!([
            199u8, 186u8, 65u8, 203u8, 212u8, 40u8, 76u8, 23u8, 189u8, 201u8, 35u8, 94u8, 83u8,
            41u8, 2u8, 23u8, 153u8, 11u8, 163u8, 93u8, 25u8, 222u8, 163u8, 42u8, 1u8, 76u8, 240u8,
            114u8, 31u8, 122u8, 80u8, 70u8
        ]);
        assert_eq!(expanded.to_string(), expected.to_string());
    }
}
