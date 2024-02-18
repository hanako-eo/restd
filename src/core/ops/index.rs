use crate::core::marker::Sized;

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}

#[lang = "index_mut"]
pub trait IndexMut<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&mut self, index: Idx) -> &Self::Output;
}
