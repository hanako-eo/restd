use crate::core::{
    clone::Clone,
    marker::{Copy, Sized},
    mem::MaybeUninit,
    option::Option,
};

#[repr(transparent)]
pub struct NonNull<T: ?Sized>(*mut T);

// TODO: implement all method for *mut T and *const T
impl<T: ?Sized> NonNull<T> {
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> NonNull<T> {
        NonNull(ptr)
    }

    #[inline]
    pub const fn new(ptr: *mut T) -> Option<NonNull<T>> {
        if ptr.is_null() {
            Option::None
        } else {
            Option::Some(unsafe { Self::new_unchecked(ptr) })
        }
    }

    // TODO: MaybeUninit
    #[inline]
    pub const unsafe fn as_uninit_ref<'a>(self) -> &'a MaybeUninit<T>
    where
        T: Sized,
    {
        unsafe { &*(self.0 as *const MaybeUninit<T>) }
    }

    #[inline]
    pub const unsafe fn as_uninit_mut<'a>(self) -> &'a mut MaybeUninit<T>
    where
        T: Sized,
    {
        unsafe { &mut *(self.0 as *mut MaybeUninit<T>) }
    }

    #[inline]
    pub fn addr(self) -> usize
    where
        T: Sized,
    {
        self.0 as usize
    }

    #[rustc_never_returns_null_ptr]
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.0
    }

    #[inline]
    pub const unsafe fn as_ref<'a>(self) -> &'a T {
        unsafe { &*self.0 }
    }

    #[inline]
    pub const unsafe fn as_mut<'a>(self) -> &'a mut T {
        unsafe { &mut *self.0 }
    }

    #[inline]
    pub const unsafe fn cast<U>(self) -> NonNull<U> {
        NonNull::new_unchecked(self.as_ptr() as *mut U)
    }

    #[inline]
    pub const unsafe fn offset(self, count: isize) -> Self
    where
        T: Sized,
    {
        NonNull::new_unchecked(self.0.offset(count))
    }

    // TODO: nightly
    // #[inline]
    // pub unsafe fn byte_offset(self, count: isize) -> Self {
    //     NonNull::new_unchecked(self.0.byte_offset(count))
    // }

    #[inline]
    pub const unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        NonNull::new_unchecked(self.0.add(count))
    }

    // TODO: nightly
    // #[inline]
    // pub unsafe fn byte_add(self, count: usize) -> Self {
    //     NonNull::new_unchecked(self.0.byte_add(count))
    // }

    #[inline]
    pub const unsafe fn sub(self, count: usize) -> Self
    where
        T: Sized,
    {
        NonNull::new_unchecked(self.0.sub(count))
    }

    // TODO: nightly
    // #[inline]
    // pub unsafe fn byte_sub(self, count: usize) -> Self {
    //     NonNull::new_unchecked(self.0.byte_sub(count))
    // }

    #[inline]
    pub const unsafe fn offset_from(self, origin: NonNull<T>) -> isize
    where
        T: Sized,
    {
        self.0.offset_from(origin.as_ptr())
    }

    // TODO: nightly
    // #[inline]
    // pub unsafe fn sub_ptr(self, subtracted: NonNull<T>) -> usize
    // where
    //     T: Sized
    // {
    //     self.0.sub_ptr(subtracted.as_ptr())
    // }

    #[inline]
    pub const unsafe fn read(self) -> T
    where
        T: Sized,
    {
        self.0.read()
    }

    #[inline]
    pub unsafe fn read_volatile(self) -> T
    where
        T: Sized,
    {
        self.0.read_volatile()
    }

    #[inline]
    pub const unsafe fn read_unaligned(self) -> T
    where
        T: Sized,
    {
        self.0.read_unaligned()
    }

    #[inline]
    pub const unsafe fn copy_to(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        self.0.copy_to(dest.as_ptr(), count)
    }

    #[inline]
    pub const unsafe fn copy_to_nonoverlapping(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        self.0.copy_to_nonoverlapping(dest.as_ptr(), count)
    }

    #[inline]
    pub const unsafe fn copy_from(self, src: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        self.0.copy_from(src.as_ptr(), count)
    }

    #[inline]
    pub const unsafe fn copy_from_nonoverlapping(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        self.0.copy_from_nonoverlapping(dest.as_ptr(), count)
    }

    #[inline]
    pub unsafe fn drop_in_place(self)
    where
        T: Sized,
    {
        self.0.drop_in_place()
    }

    #[inline]
    pub const unsafe fn write(self, value: T)
    where
        T: Sized,
    {
        self.0.write(value)
    }

    #[inline]
    pub unsafe fn write_volatile(self, value: T)
    where
        T: Sized,
    {
        self.0.write_volatile(value)
    }

    #[inline]
    pub const unsafe fn write_unaligned(self, value: T)
    where
        T: Sized,
    {
        self.0.write_unaligned(value)
    }

    #[inline]
    pub unsafe fn replace(self, value: T) -> T
    where
        T: Sized,
    {
        self.0.replace(value)
    }

    #[inline]
    pub unsafe fn swap(self, with: NonNull<T>)
    where
        T: Sized,
    {
        self.0.swap(with.as_ptr())
    }
}

impl<T: ?Sized> Clone for NonNull<T> {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: if the clone is called, we can consider than the pointer is non-null
        unsafe { NonNull::new_unchecked(self.0) }
    }
}

impl<T: ?Sized> Copy for NonNull<T> {}
