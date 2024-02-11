pub mod core;

use std::ops::Add;

pub fn add<T: Add>(a: T, b: T) -> <T as Add>::Output {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4)
}
