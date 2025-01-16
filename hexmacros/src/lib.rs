
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
mod util;
mod marker;
mod inttypes;
mod table;

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
/// - `deterministic`
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
/// for_each_int_type!(no_sized; all !sized);
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

/// Retrieves `CARGO_PKG_NAME` from environment variables.
#[proc_macro]
pub fn package_name(_: TokenStream) -> TokenStream {
    quote!(env!("CARGO_PKG_NAME")).into()
}

/// Ignores the input and does nothing. This macro has no functionality.
/// This macro is mostly for the programmer working in the editor with syntax highlighting.
/// Often when I'm designing a macro, I'll want to create a macro that consumes the tokens
/// and does nothing with them for the purpose of mapping out the macro design.
#[proc_macro]
pub fn prototype_macro(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn make_prototype_macro(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    quote!(
        macro_rules! #ident {
            ($($_:tt)*) => {};
        }
    ).into()
}

#[proc_macro]
pub fn foreach(input: TokenStream) -> TokenStream {
    let table = parse_macro_input!(input as table::TableInput);
    quote!( #table ).into()
}

/// For testing syntax.
macro_rules! discard {
    ($($_:tt)*) => {};
}

// #[allow(unused)]
// macro_rules! table {
//     ($ident:ident ! [
//         $(
//             [ $($token:tt)* ]
//         )*
//     ]) => {
//         $(
//             $ident!{$($token)*}
//         )*
//     };
//     ($ident:ident ! (
//         $(
//             ( $($token:tt)* )
//         )*
//     )) => {
//         $(
//             $ident!{$($token)*}
//         )*
//     };
//     ($ident:ident ! {
//         $(
//             { $($token:tt)* }
//         )*
//     }) => {
//         $(
//             $ident!{$($token)*}
//         )*
//     };
// }

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        // macro_rules! table_input {
        //     ($($arg:expr),*) => {
        //         $(
        //             print!("{}", $arg);
        //         )*
        //         println!();
        //     };
        // }
        // table!{
        //     table_input![
        //         ["test", "one", "two"]
        //         [1, 2, 3, 4]
        //     ]
        // }
    }
}

discard!{
    codegen!{
        // Definining a list of identifiers.
        // %ident is the syntax.
        names = [%name1, %name2, %name3];
        other_name = %other;
        // join identifiers/numbers with join
        joined = join(names[0], other_name);

        // define functions
        fn make_ident(prefix: Ident, suffix: Ident | Digits) -> Ident {
            return join(prefix, %_middle_, suffix);
        }

        // Define tokenstream
    }

    // converts a table into a series of macro inputs.
    table!{
        your_macro![
            [i32, i64]
        ]
    }

    create_table!{
        scheme:[]
        head:[""]

    }

}