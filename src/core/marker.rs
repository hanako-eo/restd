use super::clone::Clone;

pub unsafe auto trait Send {}

pub unsafe auto trait Sync {}

pub auto trait Unpin {}

#[lang = "sized"]
pub trait Sized {}

#[rustc_unsafe_specialization_marker]
#[rustc_diagnostic_item = "Copy"]
#[lang = "copy"]
pub trait Copy: Clone {}

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

pub struct PhantomPinned;

impl<T: ?Sized> Clone for PhantomData<T> {
    fn clone(&self) -> Self {
        PhantomData
    }
}

impl<T: ?Sized> Copy for PhantomData<T> {}
