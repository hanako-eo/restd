use crate::core::marker::Sized;

#[lang = "index"]
#[doc(alias = "[]")]
#[doc(alias = "[")]
#[doc(alias = "]")]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}

#[lang = "index_mut"]
#[doc(alias = "[]")]
#[doc(alias = "[")]
#[doc(alias = "]")]
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
