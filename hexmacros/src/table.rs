use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parse::Parse, Attribute, Ident, Token
};

macro_rules! discard {
    ($($_:tt)*) => {};
}

discard!{
    table!{macro table_name[
        [cols...]
        [...]
        [...etc]
    ]}
    // becomes
    macro_rules! table_name {
        ($callback:path) => {
            $callback!{
                [cols...]
                [...]
                [...etc]
            }
        };
    }
    // or
    table!{macro table_name do [
        [cols...]
        [...]
        [...etc]
    ]}
    // becomes
    macro_rules! table_name {
        ($callback:path) => {
            $callback!{ [cols...] }
            $callback!{ [...] }
            $callback!{ [...etc] }
        };
    }
}

pub struct TableInput {
    attrs: Vec<Attribute>,
    name: Ident,
    rows: Vec<TokenStream>,
}

impl Parse for TableInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        _ = input.parse::<Token![macro]>()?;
        let name = input.parse::<Ident>()?;
        let table_content;
        braced!(table_content in input);
        let mut rows = vec![];

        loop {
            if table_content.is_empty() {
                break;
            }
            let row_content;
            bracketed!(row_content in table_content);
            rows.push(row_content.parse()?);
        }

        Ok(Self {
            attrs,
            name,
            rows,
        })
    }
}

impl ToTokens for TableInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = self.attrs.as_slice();
        let name = &self.name;
        let rows = self.rows.as_slice();
        tokens.extend(quote!(
            #(#attrs)*
            macro_rules! #name {
                () => {
                    #(
                        { #rows }
                    )*
                };
                (foreach($___macro_callback:path)) => {
                    #(
                        $___macro_callback! { #rows }
                    )*
                };
                ($___macro_callback:path) => {
                    $___macro_callback! {
                        #( { #rows } )*
                    }
                };
            }
        ));
    }
}