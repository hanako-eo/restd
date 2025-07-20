#![feature(rustc_attrs)]
#![allow(internal_features)]
#![allow(unused_macros)]

#[compiler::macro_transparency(transparent)]
macro_rules! print {
    () => {}
}

fn main() {}
