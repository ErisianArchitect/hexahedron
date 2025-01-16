use quote::ToTokens;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::GenericParam;
use syn::WhereClause;
/* Outline
deterministic!(std::collections::HashMap<String, i32>);
deterministic!(
    std::collections::HashMap<String, i32>;
    <T> for &[T];
    <T> for Vec<T>;
    <T0, T1> for (T0, T1);
);
*/
use syn::{
    parse::{self, Parse}, Token, Type
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleType {
    ty: Type,
    where_: Option<WhereClause>,
}

impl Parse for SimpleType {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            where_: input.parse().ok()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GenericType {
    pub generics: Punctuated<GenericParam, Token![,]>,
    pub ty: Type,
    pub where_: Option<WhereClause>,
}

impl Parse for GenericType {
    // parse syntax such as `<T: Trait1 + Trait2 + Trait3<i32>> for Type`
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![<]>()?;
        let generics = syn::punctuated::Punctuated::parse_separated_nonempty(input)?;
        _ = input.parse::<Token![>]>()?;
        _ = input.parse::<Token![for]>()?;
        let ty = input.parse::<Type>()?;
        let where_ = if input.peek(Token![where]) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self {
            generics,
            ty,
            where_,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Line {
    SimpleType(SimpleType),
    GenericType(GenericType),
}

impl Parse for Line {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        if let Ok(generic) = input.parse::<GenericType>() {
            Ok(Line::GenericType(generic))
        } else {
            Ok(Line::SimpleType(input.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkerInput {
    pub marker_trait: syn::Path,
    pub lines: Vec<Line>,
}

impl Parse for MarkerInput {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "Empty input."));
        }
        // marker!(trait = Deterministic; <T> for &[T]);
        _ = input.parse::<Token![trait]>()?;
        _ = input.parse::<Token![=]>()?;
        let marker_trait = input.parse()?;
        _ = input.parse::<Token![;]>()?;
        if input.is_empty() {
            return Err(syn::Error::new(input.cursor().span(), "Unexpected end of input."));
        }
        let mut lines = Vec::<Line>::new();
        loop {
            if input.is_empty() {
                break;
            }
            lines.push(input.parse()?);
            if !input.is_empty() {
                _ = input.parse::<Token![;]>()?;
            }
        }
        Ok(Self { lines, marker_trait })
    }
}

impl SimpleType {
    fn to_tokens(&self, trait_path: &syn::Path, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.ty;
        let where_ = &self.where_;
        tokens.extend(quote!( impl #trait_path for #ty #where_ {} ));
    }
}

impl GenericType {
    fn to_tokens(&self, trait_path: &syn::Path, tokens: &mut proc_macro2::TokenStream) {
        let generics = &self.generics;
        let ty = &self.ty;
        let where_ = &self.where_;
        tokens.extend(quote!( impl<#generics> #trait_path for #ty #where_ {} ));
    }
}

impl Line {
    fn to_tokens(&self, trait_path: &syn::Path, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Line::SimpleType(simple_type) => simple_type.to_tokens(trait_path, tokens),
            Line::GenericType(generic_type) => generic_type.to_tokens(trait_path, tokens),
        }
    }
}

impl ToTokens for MarkerInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let lines = self.lines.as_slice();
        let trait_path = &self.marker_trait;
        lines.iter().for_each(|line| {
            line.to_tokens(trait_path, tokens);
        });
    }
}

// marker_trait

/*
marker_trait!(TraitName: Bound1 + Bound2);
marker_trait!(TraitName: Bound1 + Bound2);
marker_trait!(TraitName<Generic, T: Eq>: Bound1 + Bound2 where Generic: SomeTrait);
*/

macro_rules! marker_trait {
    ($($token:tt)*) => {};
}

marker_trait!(TraitName: Bound1 + Bound2);
marker_trait!(TraitName<Generic, T: Eq>: Bound1 + Bound2 where Generic: SomeTrait);
marker_trait!(pub(crate) TraitName: Bound1 + Bound2);
marker_trait!(pub TraitName<Generic, T: Eq>: Bound1 + Bound2 where Generic: SomeTrait);

