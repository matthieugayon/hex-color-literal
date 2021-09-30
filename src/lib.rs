extern crate proc_macro;

use core::panic;
use std::vec::IntoIter;
use quote::quote;
use proc_macro::{Delimiter, Literal, TokenStream, TokenTree};

fn ignore_groups(mut input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();
    loop {
        if let Some(TokenTree::Group(group)) = tokens.next() {
            if group.delimiter() == Delimiter::None {
                input = group.stream();
                continue;
            }
        }
        return input;
    }
}

struct TokenTreeIter {
    buf: IntoIter<u8>,
    is_punct: bool,
}

impl TokenTreeIter {
    fn new(input: Literal) -> Self {
        let mut buf: Vec<u8> = input.to_string().into();

        match buf.as_slice() {
            [b'"', .., b'"'] => (),
            _ => panic!("expected string literals"),
        };

        // remove " char
        buf.pop();
        buf.remove(0);

        Self {
            buf: buf.into_iter(),
            is_punct: false,
        }
    }

    fn next_hex_val(&mut self) -> Option<u8> {
        loop {
            let v = self.buf.next()?;

            let n = match v {
                b'0'..=b'9' => v - 48,
                b'A'..=b'F' => v - 55,
                b'a'..=b'f' => v - 87,
                b' ' | b'\r' | b'\n' | b'\t' => continue,
                _ => panic!("encountered invalid character"),
            };
            return Some(n);
        }
    }
}

impl Iterator for TokenTreeIter {
    type Item = Option<f32>;

    fn next(&mut self) -> Option<Option<f32>> {
        let v =  if self.is_punct {
            Some(None)
        } else {
            let p1 = self.next_hex_val()?;
            let p2 = match self.next_hex_val() {
                Some(v) => v,
                None => panic!("expected even number of hex characters"),
            };
            let val = (p1 << 4) + p2;
            Some(Some(val as f32 / 255.))
        };
        
        self.is_punct = !self.is_punct;

        v
    }
}

/// Macro for converting sequence an hexadecimal color string 
/// into a Color { r: f32, g: f32, b: f32 }
#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    let mut out_ts: Option<TokenTreeIter> = None;

    for tt in ignore_groups(input) {
        let iter = match tt {
            TokenTree::Literal(literal) => TokenTreeIter::new(literal),
            _ => panic!("expected string literals"),
        };
        out_ts = Some(iter);
    }

    let res: Vec<f32> = out_ts.unwrap()
        .into_iter()
        .filter(|val| val.is_some())
        .map(|val| val.unwrap())
        .collect();

    let r = res[0];
    let g = res[1];
    let b = res[2];

    let tokens = quote! {
        Color { r: #r, g: #g, b: #b, a: 1. }
    };

    tokens.into()
}