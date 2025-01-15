
use proc_macro::TokenStream;
use quote::quote;
mod util;
mod marker;
mod inttypes;

/// Marks types with marker trait.
/// 
/// Takes input in the form of:
/// ```rust, no_run
/// mark!(trait = Trait; Type);
/// mark!(trait = Trait; Type);
/// mark!(trait = Trait; Type; OtherType);
/// mark!(trait = Trait; Type where Type: std::hash::Hash);
/// mark!(trait = Trait; <T> for &[T]);
/// mark!(trait = Trait;
///     <T> for &[T];
///     <T> for Vec<T>;
///     <T> for Box<T>;
/// );
/// mark!(trait = Trait;
///     <T> for &[T] where T: std::hash::Hash;
///     <T> for Vec<T> where T: std::hash::Hash;
/// );
/// ```
#[proc_macro]
pub fn mark(input: TokenStream) -> TokenStream {
    let marker = syn::parse_macro_input!(input as marker::MarkerInput);
    quote!( #marker ).into()
}

/// Takes as input a path to a macro and optional modifiers.
/// # Example
/// ```rust,no_run
/// for_each_int_type!(path_to_macro);
/// // or
/// for_each_int_type!(path_to_macro; signed);
/// // or
/// for_each_int_type!(path_to_macro; signed !sized !(64, 128));
/// ```
/// # Modifiers
/// - `none`
/// - `all`
/// - `signed`
/// - `unsigned`
/// - `sized` (for `isize` and `usize`)
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `u128`
/// - `usize`
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `i128`
/// - `isize`
/// 
/// Each modifier can be negated using `!`.
/// ```rust, no_run
/// for_each_int_type!(no_sized; !sized);
/// ```
/// 
/// Modifiers can also be placed in groups, that can also be negated:
/// ```rust, no_run
/// for_each_int_type!(groups; signed !(isize i128) (u8 u16))
/// ```
#[proc_macro]
pub fn for_each_int_type(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as inttypes::ForEachIntTypeInput);
    quote!( #parsed ).into()
}

#[proc_macro]
pub fn crate_name(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("Unexpected tokens.");
    }
    let name = std::env::var("CARGO_PKG_NAME").expect("Failed to get CARGO_PKG_NAME");
    quote!(#name).into()
}