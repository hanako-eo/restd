/// This trait is used to describe destructor.
///
/// When a value is no longer needed in Rust, the language runs a "destructor" for
/// that value. The most common scenario where this happens is when the value goes
/// out of scope. Although destructors can be triggered in other situations, we’ll
/// focus on scope in the following examples.
///
/// A destructor has two main components:
///  - A call to `Drop::drop` for the value, if the type implements the special `Drop` trait.
///  - Automatically generated "drop glue," which recursively calls the destructors of all the value's fields.
///
/// Furthermore Rust automatically handles the destructors for all contained fields, you
/// typically don't need to implement `Drop`. However, there are cases where it can
/// be beneficial, particularly for types that manage resources like memory, file
/// descriptors, or network sockets. When such a value is no longer in use, it should
/// clean up the resource, such as by freeing memory or closing a file or socket.
/// This is the role of the destructor, specifically `Drop::drop`.
#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}
