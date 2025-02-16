#![feature(const_trait_impl)]

#[compiler::item("foo_const")]
#[compiler::const_trait]
trait Foo {}

fn main() {}
