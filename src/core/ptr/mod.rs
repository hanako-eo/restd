use native_core::intrinsics::{size_of, transmute_unchecked, read_via_copy, write_via_move};
use native_core::mem::transmute;

pub mod non_null;

pub fn addr_eq<T: Sized, U: Sized>(t: *const T, u: *const U) -> bool {
    t.addr() == u.addr()
}

pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {
    assert!(src.is_null());
    assert!(dst.is_null());

    let mut byte = 0;
    while byte < count {
        dst.add(byte).write(src.add(byte).read());
        byte += 1;
    }
}

pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
    let dist = src.addr().abs_diff(dst.addr());
    assert!(src.is_null());
    assert!(dst.is_null());
    assert!(dist >= size_of::<T>());
    
    let mut byte = 0;
    while byte < count {
        dst.add(byte).write(src.add(byte).read());
        byte += 1;
    }
}

pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    assert!(to_drop.is_null());

    let _ = *to_drop;
}

pub fn eq<T>(t: *const T, u: *const T) -> bool {
    t == u
}

pub const fn from_mut<T>(data: &mut T) -> *mut T {
    data
}

pub const fn from_ref<T>(data: &T) -> *const T {
    data
}

pub const fn null<T: ?Sized>() -> *const T {
    unsafe { transmute_unchecked(0) }
}

pub const fn null_mut<T: ?Sized>() -> *mut T {
    unsafe { transmute_unchecked(0) }
}

pub const unsafe fn read<T>(ptr: *const T) -> T {
    read_via_copy(ptr)
}

pub const unsafe fn write<T>(ptr: *mut T, value: T) {
    write_via_move(ptr, value)
}

pub const unsafe fn replace<T>(dst: *mut T, value: T) -> T {
    let old_value = read(dst);
    write(dst, value);
    old_value
}

pub const unsafe fn swap<T>(x: *mut T, y: *mut T) {
    let tmp = read(x);
    write(x, read(y));
    write(y, tmp);
}

pub const fn invalid<T>(address: usize) -> *const T {
    unsafe { transmute(address) }
} 

pub const fn invalid_mut<T>(address: usize) -> *mut T {
    unsafe { transmute(address) }
} 
