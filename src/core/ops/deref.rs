use crate::core::marker::Sized;

/// This trait is used to describe the immutable dereferencing of an object,
/// i.e. when you wish to obtain the value referenced (“pointed to”) by the object.
///
/// In the Rust language, dereferencing can be written explicitly with the unary
/// operator `*` (like `*v`). What's more, the compiler may implicit dereferences
/// (this mechanism is called "Deref coercion"), which consists in allowing the use
/// of the type put in `Target` as if the value being manipulated were of that type,
/// the compiler will then perform an automatic replacement of the form
/// `v.something()` into `Deref::deref(&v).something()`.
#[lang = "deref"]
#[doc(alias = "*")]
#[doc(alias = "&*")]
pub trait Deref {
    #[lang = "deref_target"]
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        *self
    }
}

impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        *self
    }
}

/// This trait is used to describe the mutable dereferencing of an object, i.e.
/// when you wish to modify the value referenced (“pointed to”) by the object.
///
/// In the Rust language, mutable dereferencing can be written explicitly with the
/// unary operator `*` (like `*v = 1`). What's more, the compiler may implicit
/// dereferences (this mechanism is called "Deref coercion" and in the case of DerefMut
/// it's "mutable Deref coercion"), it consists in allowing the type put in `Target`
/// to be used as if the value we're manipulating were of that type, the compiler
/// will automatically automatically replace the form `v.something()` with
/// `DerefMut::deref_mut(&mut v).something()`.
#[lang = "deref_mut"]
#[doc(alias = "*")]
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

// Not autorise to do `*a = b` when T is a immutable ref
impl<T: ?Sized> !DerefMut for &T {}

impl<T: ?Sized> DerefMut for &mut T {
    fn deref_mut(&mut self) -> &mut T {
        *self
    }
}

#[lang = "receiver"]
#[doc(hidden)]
pub trait Receiver {}
