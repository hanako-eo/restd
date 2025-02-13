#![feature(decl_macro)]

use paste::paste;
use proc_macro::TokenStream;
use quote::quote;
use syn::Error;

macro simple_compiler_rule {
    ($($(#[$($attrss:tt)*])* $rule_name:ident $(=> ($($compile_rule:ident),+))?,)+) => {$(
        simple_compiler_rule!($(#[$($attrss)*])* $rule_name $(=> ($($compile_rule),+))?);
    )*},
    ($(#[$($attrss:tt)*])* $name:ident) => {paste! {
        simple_compiler_rule!($(#[$($attrss)*])* $name => ([<rustc_ $name>]));
    }},
    ($(#[$($attrss:tt)*])* $rule_name:ident => ($($compile_rule:ident),+)) => {
        $(#[$($attrss)*])*
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
    // TRAITS RULES

    /// The `coinductive` attribute is used by the compiler during the process
    /// of [trait solving] to change the default behavior of the compiler
    /// regarding the targeted trait.
    /// 
    /// The trait solver now will use a [coinductive] method in place of an
    /// inductive solver.
    /// 
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    /// [coinductive]: https://rustc-dev-guide.rust-lang.org/solve/coinduction.html
    coinductive,
    /// The `discreet_macro_impl` attribute is used to tell the compiler do not
    /// annotate the field of the targeted implementer as used in the derived
    /// implementation.
    /// 
    /// # Example
    /// 
    /// ```compile_fail
    /// #[compiler::discreet_macro_impl]
    /// trait MyDiscreetTrait {
    ///     fn discreet(&self);
    /// }
    /// 
    /// /* some code to derive MyDiscreetTrait */
    /// 
    /// #[derive(MyDiscreetTrait)]
    /// struct Foo {
    ///     a: i32 // will generate a dead_code warning
    /// }
    /// 
    /// fn main() {
    ///     let foo = Foo { a: 2 };
    ///     foo.discreet();
    /// }
    /// ```
    discreet_macro_impl => (rustc_trivial_field_reads),
    /// The `specialization_trait` attribute is used to tell the compiler to do
    /// [specialized check] during the [trait solving] phase.
    /// 
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    /// [specialized check]: https://rustc-dev-guide.rust-lang.org/traits/specialization.html
    specialization_trait,
    /// This `unimplementable` attribute is used to tell the compiler that the
    /// targeted trait cannot be implemented by anyone.
    unimplementable => (rustc_deny_explicit_impl, rustc_do_not_implement_via_object),
    /// The `unsafe_marker` attribute is used to tell the compiler to do
    /// [specialized check] during the [trait solving] phase.
    /// 
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    /// [specialized check]: https://rustc-dev-guide.rust-lang.org/traits/specialization.html
    unsafe_marker => (unsafe_specialization_marker),

    // FUNCTIONS RULES

    /// The `nounwind` attribute refer to the [unwind] principal.
    /// It indicate to the compiler that the function cannot panic (panic-free).
    /// 
    /// [unwind]: https://doc.rust-lang.org/nomicon/unwinding.html
    nounwind,
    /// The `pure_intrinsic` attribute is used to tell the compiler that a
    /// function is an intrinsic function of the compiler.
    pure_intrinsic => (rustc_intrinsic_must_be_overridden, rustc_intrinsic),
}

// TODO: add attribute to generate
// #[cfg_attr(
//     bootstrap,
//     rustc_const_stable(feature = "const_unreachable_unchecked", since = "1.57.0")
// )]
// #[cfg_attr(not(bootstrap), rustc_intrinsic_const_stable_indirect)]
