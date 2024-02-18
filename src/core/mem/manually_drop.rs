use native_core::ops::{Deref, DerefMut};

use crate::core::ptr::{drop_in_place, read};

// #[lang = "manually_drop"]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ManuallyDrop<T> {
    value: T,
}

impl<T> ManuallyDrop<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    pub fn into_inner(self: Self) -> T {
        self.value
    }

    pub unsafe fn take(self: &mut Self) -> T {
        read(&self.value)
    }
}

impl<T> ManuallyDrop<T> {
    pub unsafe fn drop(self: &mut Self) {
        drop_in_place(&mut self.value);
    }
}

impl<T> Deref for ManuallyDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ManuallyDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
