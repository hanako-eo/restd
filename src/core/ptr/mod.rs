use super::{
    intrinsics::{read_via_copy, transmute, transmute_unchecked, write_via_move},
    marker::Sized,
};

pub mod non_null;

#[inline]
pub fn addr_eq<T: Sized, U: Sized>(t: *const T, u: *const U) -> bool {
    t.addr() == u.addr()
}

#[inline]
pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {
    // assert!(!src.is_null());
    // assert!(!dst.is_null());

    let mut byte = 0;
    while byte < count {
        dst.add(byte).write(src.add(byte).read());
        byte += 1;
    }
}

#[inline]
pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
    let dist = src.addr().abs_diff(dst.addr());
    // assert!(!src.is_null());
    // assert!(!dst.is_null());
    // assert!(dist >= size_of::<T>());

    let mut byte = 0;
    while byte < count {
        dst.add(byte).write(src.add(byte).read());
        byte += 1;
    }
}

#[inline]
pub unsafe fn drop_in_place<T>(to_drop: *mut T) {
    // assert!(!to_drop.is_null());

    // equivalent to call drop
    let _ = read(to_drop);
}

#[inline]
pub fn eq<T>(t: *const T, u: *const T) -> bool {
    t == u
}

#[inline]
pub const fn from_mut<T>(data: &mut T) -> *mut T {
    data
}

#[inline]
pub const fn from_ref<T>(data: &T) -> *const T {
    data
}

#[inline]
pub const fn null<T: ?Sized>() -> *const T {
    unsafe { transmute_unchecked(0) }
}

#[inline]
pub const fn null_mut<T: ?Sized>() -> *mut T {
    unsafe { transmute_unchecked(0) }
}

#[inline]
pub const unsafe fn read<T>(ptr: *const T) -> T {
    read_via_copy(ptr)
}

#[inline]
pub const unsafe fn write<T>(ptr: *mut T, value: T) {
    write_via_move(ptr, value)
}

#[inline]
pub const unsafe fn replace<T>(dst: *mut T, value: T) -> T {
    let old_value = read(dst);
    write(dst, value);
    old_value
}

#[inline]
pub const unsafe fn swap<T>(x: *mut T, y: *mut T) {
    let tmp = read(x);
    write(x, read(y));
    write(y, tmp);
}

#[inline]
pub const fn invalid<T>(address: usize) -> *const T {
    unsafe { transmute(address) }
}

#[inline]
pub const fn invalid_mut<T>(address: usize) -> *mut T {
    unsafe { transmute(address) }
}
