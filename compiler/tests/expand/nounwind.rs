#![feature(rustc_attrs)]

#![allow(dead_code)]
#![allow(internal_features)]

#[compiler::nounwind]
fn test_nounwind() {}

fn main() {}
