use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parenthesized, braced, bracketed, parse::Parse, token::{Brace, Bracket, Paren}, Token};


macro_rules! discard {
    ($($_:tt)*) => {};
}

discard!{
    macro_rules! prints {
        () => {};
        ($first:expr $(, $arg:expr)* $(,)?) => {
            print!("{}", $first);
            $(
                print!(", {}", $arg);
            )*
            println!();
        };
    }
    foreach!(
        prints![
            ["test", "one", "two"]
            [1, 2, 3, 4]
        ]
    );
}

pub struct ForeachInput {
    path: syn::Path,
    rows: Vec<TokenStream>,
}

impl Parse for ForeachInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::Path>()?;
        _ = input.parse::<Token![!]>()?;
        macro_rules! grouped {
            ($group_macro:path) => {
                let content;
                $group_macro!(content in input);
                let mut rows = vec![];
                let mut commas = None;
                loop {
                    // Unfortunately have to do this check twice.
                    if content.is_empty() {
                        break;
                    }
                    let row_content;
                    $group_macro!(row_content in content);
                    rows.push(row_content.parse()?);
                    if content.is_empty() {
                        break;
                    } else {
                        match commas {
                            Some(true) => {
                                _ = content.parse::<Token![,]>()?;
                            }
                            None => {
                                commas = Some(if content.peek(Token![,]) {
                                    _ = content.parse::<Token![,]>()?;
                                    true
                                } else {
                                    false
                                });
                            }
                            _ => (),
                        }
                    }
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

impl ToTokens for ForeachInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        for row in self.rows.iter() {
            tokens.extend(quote!( #path!{#row} ));
        }
    }
}