#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use lang_item::{Constraint, ItemAttribut, LangItem, Target};
use paste::paste;
use proc_macro::TokenStream;
use quote::quote;
use symbol::Symbol;
use syn::{Error, parse_macro_input};

mod lang_item;
mod symbol;

macro_rules! count_metas {
    ($first:meta, $($rest:meta),+) => { 1 + count_metas!($($rest),+) };
    ($first:meta) => { 1 };
    () => { 0 };
}

macro_rules! simple_compiler_rule {
    ($($(#[$attr:meta])* $($idents:ident)+ $(=> [$($compile_rule:meta),+])?,)+) => {$(
        simple_compiler_rule!($(#[$attr])* $($idents)* $(=> [$($compile_rule),+])?);
    )*};
    ($(#[$attr:meta])* remarkable $name:ident) => {paste! {
        simple_compiler_rule!($(#[$attr])* remarkable $name => [[<rustc_ $name>]]);
    }};
    ($(#[$attr:meta])* $name:ident) => {paste! {
        simple_compiler_rule!($(#[$attr])* $name => [[<rustc_ $name>]]);
    }};
    ($(#[$attr:meta])* remarkable $rule_name:ident => [$($compile_rule:meta),+]) => {
        paste! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub(crate) const [<$rule_name _symbols>]: [Symbol; 2 + count_metas!($($compile_rule),+)] = [Symbol(stringify!($rule_name)), Symbol(concat!("compiler::", stringify!($rule_name))), $(Symbol(stringify!($compile_rule))),+];
        }

        simple_compiler_rule!($(#[$attr])* $rule_name => [$($compile_rule),+]);
    };
    ($(#[$attr:meta])* $rule_name:ident => [$($compile_rule:meta),+]) => {
        $(#[$attr])*
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
    };
}

simple_compiler_rule! {
    // PURELY REEXPORTED RULES

    // / The `const_trait` attribute is used to tell the compiler that the targeted
    // / trait can be used in a const context. Actually used instead of the syntax
    // / `const trait MyTrait {}` (see [#67792] or [const trait RFC] for more info)
    // /
    // / [#67792]: https://github.com/rust-lang/rust/issues/67792
    // / [const trait RFC]: https://github.com/oli-obk/rfcs/blob/const-trait-impl/text/0000-const-trait-impls.md
    remarkable const_trait => [const_trait],
    /// The `fundamental` attribute is used to change the behaviour of the compiler
    /// of the targeted type on implementations.
    ///
    /// By default, the default behavior about the implementation of a type is
    /// if the type is an upstream type (as opposed to a local type, i.e. a type
    /// from the current crate) you only can implement a method into the upstream
    /// type if you do an implementation via a local trait.
    ///
    /// But an issue can occure that you need add some (upstream) trait to an
    /// uptream type with will take a generic to specialize a implementation
    /// like this:
    ///
    /// ```ignore
    /// use std::rc::Rc;
    ///
    /// struct Bar;
    ///
    /// // We try to do an specialized implementation of Default over Rc<Bar>,
    /// // but Rc is not fundamental so the compiler return an error:
    /// // error[E0117]: only traits defined in the current crate
    /// // can be implemented for arbitrary types
    /// impl Default for Rc<Bar> {
    ///     fn default() -> Self {
    ///         Rc::new(Bar)
    ///     }
    /// }
    /// ```
    ///
    /// Marked a type as fundamental tells to the [trait solver] to allow speciliazed
    /// implementation to exist.
    ///
    /// If you want more detail and info you can visit this stack overflow page:
    /// https://stackoverflow.com/questions/59022263/what-is-a-fundamental-type-in-rust
    ///
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    remarkable fundamental => [fundamental],
    remarkable full_transparent => [rustc_pub_transparent, repr(transparent)],

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
    /// ```ignore
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
    discreet_macro_impl => [rustc_trivial_field_reads],
    /// The `specialization_trait` attribute is used to tell the compiler to do
    /// [specialized check] during the [trait solving] phase.
    ///
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    /// [specialized check]: https://rustc-dev-guide.rust-lang.org/traits/specialization.html
    specialization_trait,
    /// This `unimplementable` attribute is used to tell the compiler that the
    /// targeted trait cannot have user-provided implementations.
    ///
    /// In more, `unimplementable` force rustc to opts out of the automatic
    /// trait impl for trait objects
    unimplementable => [rustc_deny_explicit_impl, rustc_do_not_implement_via_object],
    /// The `unsafe_marker` attribute is used to tell the compiler to do
    /// [specialized check] during the [trait solving] phase.
    ///
    /// [trait solving]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    /// [specialized check]: https://rustc-dev-guide.rust-lang.org/traits/specialization.html
    unsafe_marker => [unsafe_specialization_marker],

    // FUNCTIONS RULES

    /// The `nounwind` attribute refer to the [unwind] principal.
    /// It indicate to the compiler that the function cannot panic (panic-free).
    ///
    /// [unwind]: https://doc.rust-lang.org/nomicon/unwinding.html
    nounwind,
    /// The `pure_intrinsic` attribute is used to tell the compiler that a
    /// function is an intrinsic function of the compiler.
    pure_intrinsic => [rustc_intrinsic_must_be_overridden, rustc_intrinsic],
}

/// The `item` attribute is used to tell the compiler that the target is special.
///
/// Due to the compiler's operation and rust's team choice, a part of the core
/// library is not hard-coded in the compiler source code and so the idea behind
/// languages items is to allow the compiler to know the existance of some part
/// of the std via the attribute `#[lang = ""]` and allow it to operate several
/// complex things like:
///   - desugaring (like with the operator overloading, [`Range`][std::ops::Range] for example)
///   - optimise (by override the implemention by an more optimized version)
///   - alter default behavior (e.g. allow a structure to have special semantics that cannot be expressed by default)
#[proc_macro_attribute]
pub fn item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let lang_item = match LangItem::try_from(parse_macro_input!(attr as syn::LitStr)) {
        Ok(attrs) => attrs,
        Err(error) => return error.into_compile_error().into(),
    };
    let item = parse_macro_input!(item as syn::Item);

    let mut constraints = lang_item
        .constraints()
        .iter()
        .map(|constraint| (*constraint, false))
        .collect::<HashMap<_, bool>>();
    let target = lang_item.target();

    macro_rules! check_constraint {
        ($constraint:path, $token:expr) => {
            if constraints.contains_key(&$constraint) && $token.is_none() {
                return syn::Error::new_spanned(
                    item,
                    format!(
                        "`{}` item must be an {} {}",
                        lang_item.name(),
                        $constraint.name(),
                        target.name()
                    ),
                )
                .into_compile_error()
                .into();
            }
            // set the constraint to checked
            constraints.insert($constraint, true);
        };
    }
    let (generics, attrs) = match (target, &item) {
        (Target::Trait, syn::Item::Trait(item)) => {
            check_constraint!(Constraint::Auto, item.auto_token);
            check_constraint!(Constraint::Unsafety, item.unsafety);

            (&item.generics, &item.attrs)
        }
        (Target::Struct, syn::Item::Struct(item)) => (&item.generics, &item.attrs),
        (Target::Fn, syn::Item::Fn(item)) => {
            check_constraint!(Constraint::Constness, item.sig.constness);
            check_constraint!(Constraint::Unsafety, item.sig.unsafety);

            let args_len = item.sig.inputs.len();
            let argument_constraint = constraints
                .iter()
                .find(|(constraint, _)| matches!(constraint, Constraint::Arguments(_)))
                .map(|(c, _)| *c)
                .unwrap_or(Constraint::Arguments(0));

            if let Constraint::Arguments(n) = argument_constraint
                && n != args_len
            {
                // TODO: change msg to avoid '1 arguments'
                return syn::Error::new_spanned(&item.sig, format!("`{}` item expected to have {1} arguments but the definition has {2} arguments\n please force the function to have {1} arguments", lang_item.name(), n, args_len)).into_compile_error().into();
            }
            constraints.insert(argument_constraint, true);

            (&item.sig.generics, &item.attrs)
        }
        (Target::Enum, syn::Item::Enum(item)) => (&item.generics, &item.attrs),
        (Target::Union, syn::Item::Union(item)) => (&item.generics, &item.attrs),
        (target, _) => {
            return syn::Error::new_spanned(
                item,
                format!(
                    "`{}` item must be applied to a {} item",
                    lang_item.name(),
                    target.name()
                ),
            )
            .into_compile_error()
            .into();
        }
    };

    let generic_len = generics.params.len();
    let generic_constraint = constraints
        .iter()
        .find(|(constraint, _)| matches!(constraint, Constraint::Generics(_)))
        .map(|(c, _)| *c)
        .unwrap_or(Constraint::Generics(0));
    if let Constraint::Generics(n) = generic_constraint
        && n != generic_len
    {
        // TODO: change msg to avoid '1 generics'
        return syn::Error::new_spanned(item, format!("`{}` item expected to have {1} generics but the definition has {2} generics\n please force the {3} item to have {1} generics", lang_item.name(), n, generic_len, target.name())).into_compile_error().into();
    }

    let items_attrs = attrs
        .iter()
        .map(|attr| ItemAttribut::from(attr.meta.clone()))
        .filter(|item| item != &ItemAttribut::Unknown)
        .collect::<HashSet<_>>();
    for (constraint, checked) in constraints {
        if checked {
            continue;
        }

        let checked = match constraint {
            Constraint::Auto
            | Constraint::Unsafety
            | Constraint::Generics(_)
            | Constraint::Arguments(_) => false,
            Constraint::Constness => items_attrs.contains(&ItemAttribut::ConstTrait),
            Constraint::Fundamental => items_attrs.contains(&ItemAttribut::Fundamental),
            Constraint::Transparent => items_attrs.contains(&ItemAttribut::Transparent),
        };

        if !checked {
            return syn::Error::new_spanned(
                item,
                format!(
                    "`{}` item must be a {} {}",
                    lang_item.name(),
                    constraint.name(),
                    target.name()
                ),
            )
            .into_compile_error()
            .into();
        }
    }

    quote! {
        #lang_item
        #item
    }
    .into()
}
