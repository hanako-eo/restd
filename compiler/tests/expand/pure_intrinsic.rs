#![feature(rustc_attrs)]
#![feature(intrinsics)]

#![allow(dead_code)]
#![allow(internal_features)]

#[compiler::pure_intrinsic]
const fn size_of<T>() -> usize { 0 }

fn main() {}
