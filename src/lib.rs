// FEATURES
#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(auto_traits)]
#![feature(const_extern_fn)]
#![feature(const_for)]
// #![feature(const_intrinsic_copy)]
#![feature(const_mut_refs)]
// #![feature(const_ptr_is_null)]
// #![feature(const_ptr_write)]
// #![feature(core_intrinsics)]
#![feature(f16)]
#![feature(f128)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(no_core)]
// #![feature(ptr_metadata)]
#![feature(rustc_attrs)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(strict_provenance)]
#![feature(transparent_unions)]
#![feature(unboxed_closures)]
// END FEATURES
#![allow(internal_features)]
#![allow(incomplete_features)]
// Remove the binding to std
#![no_std]
// Remove the binding to rust core
#![no_core]

pub mod core;

// TODO: ADD STABILITY ATTRIBUT

// pub fn add<T: Add>(a: T, b: T) -> <T as Add>::Output {
//     a + b
// }

// #[test]
// fn test_add() {
//     assert_eq!(add(2, 2), 4)
// }
