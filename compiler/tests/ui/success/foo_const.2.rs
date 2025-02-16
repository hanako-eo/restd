#![feature(const_trait_impl)]

#[const_trait]
#[compiler::item("foo_const")]
trait Foo {}

fn main() {}
