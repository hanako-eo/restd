use super::clone::Clone;

// #[lang = "send"]
pub unsafe auto trait Send {}

#[lang = "sync"]
pub unsafe auto trait Sync {}

#[rustc_diagnostic_item = "Unpin"]
#[lang = "unpin"]
pub auto trait Unpin {}

#[lang = "sized"]
pub trait Sized {}

#[rustc_unsafe_specialization_marker]
#[rustc_diagnostic_item = "Copy"]
#[lang = "copy"]
pub trait Copy: Clone {}

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

impl<T: ?Sized> Clone for PhantomData<T> {
    fn clone(&self) -> Self {
        Self
    }
}

impl<T: ?Sized> Copy for PhantomData<T> {}

pub struct PhantomPinned;
