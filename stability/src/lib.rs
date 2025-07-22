use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Error, parse_macro_input};

use crate::internals::{DeprecatedAttribute, StableAttribute, UnstableAttribute};

mod internals;

fn merge_unsetted_args(args: Vec<String>) -> Option<String> {
    let len = args.len();
    let mut iter = args.into_iter();

    let first = iter.next()?;
    Some(format!(
        "{} need to be set",
        iter.enumerate()
            .fold(first, |acc, (i, s)| match i == len - 2 {
                true => format!("{acc} and {s}"),
                false => format!("{acc}, {s}"),
            })
    ))
}

#[proc_macro_attribute]
pub fn stable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let StableAttribute {
        ctx,

        constness,
        feature,
        since,
    } = parse_macro_input!(attr with StableAttribute::parse);
    if let Err(err) = ctx.check(merge_unsetted_args) {
        return err.into_compile_error().into();
    }

    match constness {
        true => quote! {
            #[rustc_const_stable(feature = #feature, since = #since)]
            #item
        },
        false => quote! {
            #[stable(feature = #feature, since = #since)]
            #item
        },
    }
    .into()
}

#[proc_macro_attribute]
pub fn unstable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let UnstableAttribute {
        ctx,

        constness,
        stabilisable,
        intrinsic,

        feature,
        issue,
        reason,
    } = parse_macro_input!(attr with UnstableAttribute::parse);
    if let Err(err) = ctx.check(merge_unsetted_args) {
        return err.into_compile_error().into();
    }

    match (constness, stabilisable, intrinsic) {
        (_, true, true) => Error::new(
            Span::call_site(),
            "cannot have intrinsic and stabilisable at the same time",
        )
        .into_compile_error(),
        (true, false, true) => quote! {
            #[rustc_const_unstable(feature = #feature, issue = #issue, reason = #reason)]
            #[rustc_intrinsic_const_stable_indirect]
            #item
        },
        (true, true, false) => quote! {
            #[rustc_const_unstable(feature = #feature, issue = #issue, reason = #reason)]
            #[rustc_const_stable_indirect]
            #item
        },
        (true, false, false) => quote! {
            #[rustc_const_unstable(feature = #feature, issue = #issue, reason = #reason)]
            #item
        },
        (false, _, _) => quote! {
            #[unstable(feature = #feature, issue = #issue, reason = #reason)]
            #item
        },
    }
    .into()
}

#[proc_macro_attribute]
pub fn deprecated(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let DeprecatedAttribute {
        ctx,

        since,
        note,
        suggestion,
    } = parse_macro_input!(attr with DeprecatedAttribute::parse);
    if let Err(err) = ctx.check(merge_unsetted_args) {
        return err.into_compile_error().into();
    }

    quote! {
        #[deprecated(
            since = #since,
            note = #note,
            suggestion = #suggestion,
        )]
        #item
    }
    .into()
}
