use super::manually_drop::ManuallyDrop;
use crate::core::clone::Clone;
use crate::core::intrinsics::{size_of, transmute_unchecked};
use crate::core::marker::Copy;
use crate::core::ptr::{drop_in_place, read, write};

#[repr(transparent)]
pub union MaybeUninit<T> {
    // TODO: replace me with super::manually_drop::ManuallyDrop
    value: ManuallyDrop<T>,
    uninit: (),
}

impl<T> MaybeUninit<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
        }
    }

    pub const fn uninit() -> Self {
        Self { uninit: () }
    }

    pub const fn uninit_array<const N: usize>() -> [Self; N] {
        unsafe { MaybeUninit::<[Self; N]>::uninit().assume_init() }
    }

    pub const fn zeroed() -> Self
    where
        [(); size_of::<T>()]:,
    {
        let mut uninit = Self::uninit();
        unsafe {
            write(
                uninit.as_mut_ptr() as *mut [u8; size_of::<T>()],
                [0; size_of::<T>()],
            )
        };
        uninit
    }

    pub const fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    pub fn write(&mut self, value: T) -> &mut T {
        let ptr = self.as_mut_ptr();
        unsafe {
            write(ptr, value);
            &mut *ptr
        }
    }

    pub const unsafe fn assume_init(self) -> T {
        ManuallyDrop::into_inner(self.value)
    }

    pub const unsafe fn assume_init_read(&self) -> T {
        read(self.as_ptr())
    }

    pub unsafe fn assume_init_drop(&mut self) {
        drop_in_place(self.as_mut_ptr());
    }

    pub unsafe fn assume_init_ref(&mut self) -> &T {
        &*self.as_ptr()
    }

    pub unsafe fn assume_init_mut(&mut self) -> &mut T {
        &mut *self.as_mut_ptr()
    }
}

impl<T, const N: usize> MaybeUninit<[T; N]> {
    pub const fn transpose(self) -> [MaybeUninit<T>; N] {
        unsafe { transmute_unchecked(self) }
    }
}

impl<T: Copy> Clone for MaybeUninit<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for MaybeUninit<T> {}
