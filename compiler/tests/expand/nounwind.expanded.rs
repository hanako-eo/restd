#![feature(rustc_attrs)]
#![allow(dead_code)]
#![allow(internal_features)]
#[rustc_nounwind]
fn test_nounwind() {}
fn main() {}
