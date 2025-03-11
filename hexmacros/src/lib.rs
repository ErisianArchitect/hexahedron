
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Attribute};
mod util;
mod marker;
mod inttypes;
mod foreach;
mod table;
mod define;

/// Marks types with marker trait.
/// 
/// Takes input in the form of:
/// ```rust, no_run
/// mark!(trait = Trait; Type);
/// mark!(trait = Trait; Type);
/// mark!(trait = Trait; Type; OtherType);
/// // Optional `where` clause.
/// mark!(trait = Trait; Type where Type: std::hash::Hash); // (Type is not generic here)
/// // Use `<GenericName> for <TypeWithGeneric<GenericName>>` syntax for generics.
/// mark!(trait = Trait; <T> for &[T]);
/// mark!(trait = Trait;
///     <T> for &[T];
///     <T> for Vec<T>;
///     <T> for Box<T>;
/// );
/// // Optional `where` clause.
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
/// - `deterministic` (all types besides `isize` and `usize`)
/// - `sized` (`isize` and `usize`)
/// - `signed`
/// - `unsigned`
/// - `8` (`u8` and `i8`)
/// - `16` (`u16` and `i16`)
/// - `32` (`u32` and `i32`)
/// - `64` (`u64` and `i64`)
/// - `128` (`u128` and `i128`)
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
/// When adding modifiers, the modifiers start out with no types applied.
/// You must add them with modifiers. A good start is with the `all` mask,
/// which sets all the flags. Then you can remove flags that you don't want.
#[proc_macro]
pub fn for_each_int_type(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as inttypes::ForEachIntTypeInput);
    quote!( #parsed ).into()
}

/// Retrieves `CARGO_PKG_NAME` from environment variables.
/// Equivalent to `env!("CARGO_PKG_NAME")`. That's what it resolves to.
/// Essentially useless. I just made it because I could.
#[proc_macro]
pub fn package_name(_: TokenStream) -> TokenStream {
    quote!(env!("CARGO_PKG_NAME")).into()
}

/// Ignores the input and does nothing. This macro has no functionality.
/// This macro is mostly for the programmer working in the editor with syntax highlighting.
/// Often when I'm designing a macro, I'll want to create a macro that consumes the tokens
/// and does nothing with them for the purpose of mapping out the macro design.
#[proc_macro]
pub fn prototype(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Makes a prototypes macro. A prototype macro is a macro that accepts
/// absolutely any input but returns an empty token stream.
/// ```rust, no_run
/// prototype_macro!(
///     /// You can have doc comments on your macro. Or even attributes.
///     #[macro_export]
///     macro_name
/// );
/// ```
#[proc_macro]
pub fn prototype_macro(input: TokenStream) -> TokenStream {
    struct PrototypeInput {
        attrs: Vec<Attribute>,
        ident: syn::Ident,
    }
    impl Parse for PrototypeInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self {
                attrs: input.call(Attribute::parse_outer)?,
                ident: input.parse()?
            })
        }
    }
    let proto = parse_macro_input!(input as PrototypeInput);
    let attrs = proto.attrs;
    let ident = proto.ident;
    quote!(
        #(#attrs)*
        macro_rules! #ident {
            ($($_:tt)*) => {};
        }
    ).into()
}

/// ```rust, no_run
/// foreach!(macro_to_call!(
///     (any tokens here as input to macro_to_call)
///     (as many rows as you want)
///     (each row is called sequentially)
/// ))
/// ```
/// If you use `()` for the call, you use `()` for the row. Same for `[]` and `{}`.
/// 
/// Separating rows with commas is optional, but if you use commas, you must be consistent.
#[proc_macro]
pub fn foreach(input: TokenStream) -> TokenStream {
    let foreach_input = parse_macro_input!(input as foreach::ForeachInput);
    quote!( #foreach_input ).into()
}

/// ```rust, no_run
/// table!(
///     /// You can also add doc comments and attributes.
///     /// (If you need to add attributes to the macro, this is the only way.)
///     #[macro_export]
///     macro table_name {
///         [row input goes here]
///         [as many rows as you want]
///         [use [] for rows always]
///     }
/// );
/// ```
/// This will create a macro like this:
/// ```rust, no_run
/// #[doc = "You can also add doc comments and attributes.\n(If you need to add attributes to the macro, this is the only way.)"]
/// #[macro_export]
/// macro_rules! table_name {
///     () => {
///         {row input goes here}
///         {as many rows as you want}
///         {use [] for rows always}
///     };
///     (foreach($___macro_callback:path)) => {
///         $___macro_callback!{ row input goes here }
///         $___macro_callback!{ as many rows as you want }
///         $___macro_callback!{ use [] for rows always }
///     };
///     ($___macro_callback:path) => {
///         $___macro_callback!{
///             {row input goes here}
///             {as many rows as you want}
///             {use [] for rows always}
///         }
///     };
/// }
/// ```
/// The created macro can be called like so:
/// ```rust, no_run
/// table_name!();
/// // or
/// table_name!(path_to::other_macro);
/// // or
/// table_name!(foreach(path_to::other_macro));
/// ```
/// 
/// Table rows can be matched using syntax like so when not using `foreach` mode:
/// ```rust, no_run
/// $( { $($token:tt)* } )*
/// ```
/// With `foreach` mode, you can use the pattern of the row input itself.
#[proc_macro]
pub fn table(input: TokenStream) -> TokenStream {
    let table_input = parse_macro_input!(input as table::TableInput);
    quote!( #table_input ).into()
}

/// Create a `macro_rules` macro that takes no input and just outputs some tokens.
/// ```rust, no_run
/// define!(
///     /// You can have doc comments and attributes on the macro.
///     #[macro_export]
///     macro macro_name {
///         // macro output goes here
///     }
/// );
/// ```
#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let define_input = parse_macro_input!(input as define::DefineInput);
    quote!( #define_input ).into()
}

/// For testing syntax.
macro_rules! discard {
    ($($_:tt)*) => {};
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