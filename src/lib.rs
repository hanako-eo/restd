// FEATURES
#![feature(const_for)]
#![feature(const_intrinsic_copy)]
#![feature(const_mut_refs)]
#![feature(const_ptr_is_null)]
#![feature(const_ptr_write)]
#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(ptr_metadata)]
#![feature(rustc_attrs)]
#![feature(strict_provenance)]
#![feature(transparent_unions)]
// END FEATURES
#![allow(internal_features)]
#![allow(incomplete_features)]
// Remove the binding to std
#![no_std]
// Remove the binding to rust core, not actually activate to continu to have useful core
// functions, traits and more
// #![no_core]

// Make an alias of the rust core lib into native_core
extern crate core as native_core;

use native_core::ops::Add;

pub mod core;

pub fn add<T: Add>(a: T, b: T) -> <T as Add>::Output {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4)
}
