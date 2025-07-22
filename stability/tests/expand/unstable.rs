#![feature(staged_api)]
#![stable(feature = "root", since = "1.0.0")]
#![allow(internal_features)]

#[stability::stable(feature = "test", since = "1.0.0")]
#[stability::unstable(feature = "test", issue = "none", reason = "sd", constness)]
const fn test1() {}

#[stability::unstable(feature = "test", issue = "none", reason = "sd", constness, intrinsic)]
const fn test2() {}

#[stability::unstable(
    feature = "test",
    issue = "none",
    reason = "sd",
    constness,
    stabilisable
)]
const fn test3() {}

fn main() {}
