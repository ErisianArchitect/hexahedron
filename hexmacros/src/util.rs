#![allow(unused)]
use syn::{Ident, Type};
use hashbrown::HashSet;


pub fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

pub fn is_tuple(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(_))
}

pub fn is_ident_ty(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.path.get_ident().is_some())
}

pub fn get_ident(ty: &Type) -> Option<&Ident> {
    let Type::Path(path) = ty else {
        return None;
    };
    path.path.get_ident()
}

pub fn get_ident_str(ty: &Type) -> Option<String> {
    get_ident(ty).map(|ident| {
        ident.to_string()
    })
}

pub struct TypeInfo;

pub trait HasType<T> {
    fn has_type(token: &T, ty: &Type) -> bool;
}

impl HasType<Type> for TypeInfo {
    fn has_type(token: &Type, ty: &Type) -> bool {
        if token == ty {
            return true;
        }
        match token {
            Type::Array(type_array) => {
                if *type_array.elem == *ty {
                    return true;
                }
            },
            Type::BareFn(type_bare_fn) => {
                if type_bare_fn.inputs.iter().find(|input| input.ty == *ty).is_some() {
                    return true;
                }
                match &type_bare_fn.output {
                    syn::ReturnType::Default => {
                        if is_unit_type(token) {
                            return true;
                        }
                    },
                    syn::ReturnType::Type(_, ret) => if **ret == *ty {
                        return true;
                    },
                }
            },
            Type::Group(type_group) => todo!(),
            Type::ImplTrait(type_impl_trait) => todo!(),
            Type::Infer(type_infer) => todo!(),
            Type::Macro(type_macro) => todo!(),
            Type::Never(type_never) => todo!(),
            Type::Paren(type_paren) => todo!(),
            Type::Path(type_path) => todo!(),
            Type::Ptr(type_ptr) => todo!(),
            Type::Reference(type_reference) => todo!(),
            Type::Slice(type_slice) => todo!(),
            Type::TraitObject(type_trait_object) => todo!(),
            Type::Tuple(type_tuple) => todo!(),
            Type::Verbatim(token_stream) => todo!(),
            _ => todo!(),
        }
        false
    }
}

// pub trait ExtractTypes<T> {
//     fn extract_types(token: &T, set: &mut HashSet<Type>);
// }

// impl ExtractTypes<Type> for TypeInfo {
//     fn extract_types(token: &Type, set: &mut HashSet<Type>) {
//         set.insert(token.clone());
//         match token {
//             Type::Array(type_array) => {
//                 set.insert(*type_array.elem.clone());
//             },
//             Type::BareFn(type_bare_fn) => {
//                 type_bare_fn.inputs.iter().for_each(|input| {
//                     set.insert(input.ty.clone());
//                 });
//                 match &type_bare_fn.output {
//                     syn::ReturnType::Default => todo!(),
//                     syn::ReturnType::Type(_, ty) => { set.insert(*ty.clone()); },
//                 }
//             },
//             Type::Group(type_group) => {
//                 // buffer.push(type_group.elem.clone());
//                 set.insert(*type_group.elem.clone());
//             },
//             Type::ImplTrait(type_impl_trait) => {
                
                
//             },
//             Type::Infer(type_infer) => todo!(),
//             Type::Macro(type_macro) => todo!(),
//             Type::Never(type_never) => todo!(),
//             Type::Paren(type_paren) => todo!(),
//             Type::Path(type_path) => todo!(),
//             Type::Ptr(type_ptr) => todo!(),
//             Type::Reference(type_reference) => todo!(),
//             Type::Slice(type_slice) => todo!(),
//             Type::TraitObject(type_trait_object) => todo!(),
//             Type::Tuple(type_tuple) => todo!(),
//             Type::Verbatim(token_stream) => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// fn extract_types(ty: &Type, out: &mut Vec<Type>) {
    
// }