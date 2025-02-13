#![feature(decl_macro)]

use paste::paste;
use proc_macro::TokenStream;
use quote::quote;
use syn::Error;

macro simple_compiler_rule {
    ($($rule_name:ident $(=> ($($compile_rule:ident),+))?,)+) => {$(
        simple_compiler_rule!($rule_name $(=> ($($compile_rule),+))?);
    )*},
    ($name:ident) => {paste! {
        simple_compiler_rule!($name => ([<rustc_ $name>]));
    }},
    ($rule_name:ident => ($($compile_rule:ident),+)) => {
        #[proc_macro_attribute]
        pub fn $rule_name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let attr = proc_macro2::TokenStream::from(attr);
            let item = proc_macro2::TokenStream::from(item);
        
            match attr.is_empty() {
                false => Error::new_spanned(attr, concat!("must be of the form: `#[compiler::", stringify!($rule_name), "]`")).into_compile_error(),
                true => quote! {
                    $(#[$compile_rule])+
                    #item
                }
            }.into()
        }
    }
}

simple_compiler_rule! {
    coinductive,
    nounwind,
    discreet_impl => (rustc_trivial_field_reads),
    pure_intrinsic => (rustc_intrinsic_must_be_overridden, rustc_intrinsic),
}

// TODO: add attribute to generate
// #[cfg_attr(
//     bootstrap,
//     rustc_const_stable(feature = "const_unreachable_unchecked", since = "1.57.0")
// )]
// #[cfg_attr(not(bootstrap), rustc_intrinsic_const_stable_indirect)]
