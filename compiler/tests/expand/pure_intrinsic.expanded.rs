#![feature(rustc_attrs)]
#![feature(intrinsics)]
#![allow(dead_code)]
#![allow(internal_features)]
#[rustc_intrinsic_must_be_overridden]
#[rustc_intrinsic]
const fn size_of<T>() -> usize {
    0
}
fn main() {}
