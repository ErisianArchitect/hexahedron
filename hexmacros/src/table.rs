use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parenthesized, braced, bracketed, parse::Parse, token::{Brace, Bracket, Paren}, Token};


macro_rules! discard {
    ($($_:tt)*) => {};
}

discard!{
    macro_rules! table_input {
        ($($arg:expr),*) => {
            $(
                print!("{}", $arg);
            )*
            println!();
        };
    }
    table!{
        table_input![
            ["test", "one", "two"]
            [1, 2, 3, 4]
        ]
    }
}

pub struct TableInput {
    path: syn::Path,
    rows: Vec<TokenStream>,
}

impl Parse for TableInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::Path>()?;
        _ = input.parse::<Token![!]>()?;
        macro_rules! grouped {
            ($group_macro:path) => {
                let content;
                $group_macro!(content in input);
                let mut rows = vec![];
                loop {
                    if content.is_empty() {
                        break;
                    }
                    let row_content;
                    $group_macro!(row_content in content);
                    rows.push(row_content.parse()?);
                }
                rows
            };
        }
        let rows = if input.peek(Paren) {
            grouped!{parenthesized}
        } else if input.peek(Bracket) {
            grouped!{bracketed}
        } else if input.peek(Brace) {
            grouped!{braced}
        } else {
            return Err(syn::Error::new(input.cursor().span(), "Expected [...], (...), or {...}."));
        };
        Ok(Self {
            path,
            rows,
        })
    }
}

impl ToTokens for TableInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        for row in self.rows.iter() {
            tokens.extend(quote!( #path!{#row} ));
        }
    }
}