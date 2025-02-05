// TODO: need doc to explain all of that
#![unstable(feature = "core", reason = "work in progress", issue = "none")]

// Remove the binding to std
#![no_std]
// Remove the binding to rust core
#![no_core]
#![rustc_coherence_is_core]
#![rustc_preserve_ub_checks]

// mandatory features to rewrite the core
#![feature(allow_internal_unstable)]
#![feature(no_core)]
#![feature(rustc_attrs)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(negative_impls)]
// to allow to use stabilities attributes
#![feature(staged_api)]
// to allow to use #[prelude_import]
#![feature(prelude_import)]
// to allow to use macro as `macro macro_name { ... }`
#![feature(decl_macro)]

// temporary features (only used during the construction of the core)
#![feature(core_panicking_macro)]

// to avoid warning like 'the feature `staged_api` is internal to the compiler ...'
#![allow(internal_features)]


#[prelude_import]
#[allow(unused_imports)]
use prelude::rust_2021::*;

/* The core prelude, not as all-encompassing as the std prelude */
pub mod prelude;

// pub mod clone;
// pub mod default;
pub mod panic;
pub mod intrinsics;
#[macro_use]
mod macros;
// pub mod marker;
// pub mod mem;
// pub mod ops;
// pub mod option;
// pub mod ptr;
// pub mod result;
