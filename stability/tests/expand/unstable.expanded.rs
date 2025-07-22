#![feature(staged_api)]
#![stable(feature = "root", since = "1.0.0")]
#![allow(internal_features)]
#[rustc_const_unstable(feature = "test", issue = "none", reason = "sd")]
#[stable(feature = "test", since = "1.0.0")]
const fn test1() {}
#[rustc_const_unstable(feature = "test", issue = "none", reason = "sd")]
#[rustc_intrinsic_const_stable_indirect]
const fn test2() {}
#[rustc_const_unstable(feature = "test", issue = "none", reason = "sd")]
#[rustc_const_stable_indirect]
const fn test3() {}
fn main() {}
