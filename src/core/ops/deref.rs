use crate::core::marker::Sized;

#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

#[lang = "receiver"]
#[doc(hidden)]
pub trait Receiver {}
