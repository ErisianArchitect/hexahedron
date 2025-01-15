/*
for_each_int_type!(path::to::macro);
for_each_int_type!(path::to::macro; signed);
for_each_int_type!(path::to::macro; unsigned);
for_each_int_type!(path::to::macro; signed !size !(8, 16, 32, 64, 128) +(u8, u16, u32, u64) +u128 );
for_each_int_type!(path::to::macro; signed);
for_each_int_type!(path::to::macro; unsigned);
*/
use quote::quote;
use quote::ToTokens;
use syn::{parse::Parse, Ident, Token};

macro_rules! flag_names {
    ($($name:tt),+$(,)?) => {
        [
            $(
                stringify!($name),
            )*
        ]
    };
}

macro_rules! make_flags {
    ($($bit:literal: $flag:tt),*$(,)?) => {
        #[allow(unused)]
        const FLAG_NAMES: &[&'static str] = &flag_names![$($flag),*];

        pub fn get_flag(name: &str) -> u16 {
            match name {
                $(
                    stringify!($flag) => $bit,
                )*
                _ => 0,
            }
        }

        // pub fn get_flag_names(bits: u32) -> Vec<&'static str> {
        //     let mut names = Vec::new();
        //     let mut remainder = bits;
        //     // (next_bit, remaining_bits)
        //     fn next_bit_index(bits: &mut u32) -> u32 {
        //         let index = bits.trailing_zeros();
        //         let bit = 1u32 << index;
        //         let invbit = !bit;
        //         *bits = *bits & invbit;
        //         index
        //     }
        //     while remainder != 0 {
        //         let index = next_bit_index(&mut remainder);
        //         names.push(FLAG_NAMES[index as usize]);
        //     }
        //     names
        // }
    };
}

make_flags![
    0x001: u8,
    0x002: u16,
    0x004: u32,
    0x008: u64,
    0x010: u128,
    0x020: usize,
    0x040: i8,
    0x080: i16,
    0x100: i32,
    0x200: i64,
    0x400: i128,
    0x800: isize,
    0x041: 8,
    0x082: 16,
    0x104: 32,
    0x208: 64,
    0x410: 128,
    0x000: none,
    0xfff: all,
    0x03f: unsigned,
    0xfc0: signed,
    0x820: sized,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flags(pub u16);

pub struct FlagsIter {
    flags: u16,
}

impl Iterator for FlagsIter {
    type Item = &'static str;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let flags = self.flags & 0xfff;
        let trailing = flags.trailing_zeros();
        let count = (16 - trailing) as usize;
        (count, Some(count))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let bit_index = self.flags.trailing_zeros();
        (bit_index < 12).then(|| {
            self.flags = self.flags & !(1 << bit_index);
            FLAG_NAMES[bit_index as usize]
        })
    }
}

impl Flags {
    pub const fn iter(self) -> FlagsIter {
        FlagsIter {
            flags: self.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FlagModifier {
    Include(u16),
    Exclude(u16),
}

impl FlagModifier {
    // const UNMODIFIED: FlagModifier = FlagModifier::Include(0);
    // const ALL: FlagModifier = FlagModifier::Include(0xfff);
    // const EXCLUDE_ALL: FlagModifier = FlagModifier::Exclude(0xfff);
    // const SIGNED: FlagModifier = FlagModifier::Include(0xfc0);
    // const UNSIGNED: FlagModifier = FlagModifier::Include(0x03f);
    // const NO_SIZED: FlagModifier = FlagModifier::Exclude(0x820);
    // const SIZED: FlagModifier = FlagModifier::Include(0x820);

    // pub const fn new() -> Self {
    //     Self::UNMODIFIED
    // }

    pub const fn modify(self, flags: u16) -> u16 {
        match self {
            FlagModifier::Include(include) => flags | include,
            FlagModifier::Exclude(exclude) => flags & !exclude,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FlagGroup(u16);

impl FlagGroup {
    pub fn new() -> Self {
        Self(0)
    }

    pub const fn add_flag(&mut self, flag: Flag) {
        self.0 |= flag.0;
    }
}

impl Parse for FlagGroup {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let mut group = FlagGroup::new();
        loop {
            if content.is_empty() {
                break;
            }
            let modifier = content.parse::<Flag>()?;
            group.add_flag(modifier);
        }
        Ok(group)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Flag(u16);

impl Parse for Flag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (flag_name, span) = if let Ok(ident) = input.parse::<syn::Ident>() {
            (
                ident.to_string(),
                ident.span()
            )
        } else {
            let num = input.parse::<syn::LitInt>()?;
            if !num.suffix().is_empty() {
                return Err(syn::Error::new(num.span(), format!("Invalid flag input. Expected raw integer, got suffixed integer. {num}")));
            }
            (
                num.base10_digits().to_owned(),
                num.span()
            )
        };
        let flag = get_flag(&flag_name);
        if flag == 0 {
            return Err(syn::Error::new(span, format!("Not a valid flag: \"{flag_name}\".")));
        }
        Ok(Flag(flag))
    }
}

impl Parse for FlagModifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let invert = input.parse::<Token![!]>().is_ok();
        if let Ok(group) = input.parse::<FlagGroup>() {
            Ok(if invert { Self::Exclude(group.0) } else { Self::Include(group.0) })
        } else {
            let flag = input.parse::<Flag>()?;
            Ok(if invert { Self::Exclude(flag.0) } else { Self::Include(flag.0) })
        }
    }
}

impl Parse for Flags {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // for_each_int_type!(path::to::macro; signed !(64, 128, isize));
        // i8, i16, i32
        let mut flags = 0u16;
        loop {
            if input.is_empty() {
                break;
            }
            let flag = input.parse::<FlagModifier>()?;
            flags = flag.modify(flags);
        }
        Ok(Self(flags))
    }
}

pub struct ForEachIntTypeInput {
    pub path: syn::Path,
    pub flags: Flags,
}

impl Parse for ForEachIntTypeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let flags = if input.parse::<Token![;]>().is_ok() {
            input.parse()?
        } else {
            Flags(0xfff)
        };
        Ok(Self {
            path,
            flags,
        })
    }
}

impl ToTokens for ForEachIntTypeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.path;
        let lines = self.flags.iter().map(|flag| {
            let ident = Ident::new(flag, proc_macro2::Span::call_site());
            quote!( #path!{#ident} )
        }).collect::<Vec<_>>();
        tokens.extend(quote!( #( #lines )* ));
    }
}