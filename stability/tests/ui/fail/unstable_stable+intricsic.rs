#![feature(staged_api)]
#![stable(feature = "root", since = "1.0.0")]
#![allow(internal_features)]

#[stability::unstable(
    feature = "test",
    issue = "none",
    reason = "sd",
    constness,
    stabilisable,
    intrinsic
)]
const fn test3() {}

fn main() {}
