
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, Attribute, Block, Ident, Token
};

/*

*/

pub struct DefineInput {
    attrs: Vec<Attribute>,
    name: Ident,
    block: Block,
}

impl Parse for DefineInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        _ = input.parse::<Token![macro]>()?;
        let name = input.parse()?;
        let block = input.parse()?;
        Ok(Self {
            attrs,
            name,
            block,
        })
    }
}

impl ToTokens for DefineInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = self.attrs.as_slice();
        let name = &self.name;
        let block = &self.block;
        tokens.extend(quote!(
            #(#attrs)*
            macro_rules! #name {
                () => #block;
            }
        ));
    }
}