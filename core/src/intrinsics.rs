// from the corresponding core file:
// https://github.com/rust-lang/rust/blob/master/library/core/src/intrinsics/mod.rs
#![unstable(
    feature = "core_intrinsics",
    reason = "intrinsics are unlikely to ever be stabilized, instead \
                      they should be used through stabilized interfaces \
                      in the rest of the standard library",
    issue = "none"
)]

use crate::unreachable;

/// The size of a type in bytes.
///
/// Note that, unlike most intrinsics, this is safe to call;
/// it does not require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold
/// any safety invariants.
///
/// More specifically, this is the offset in bytes between successive
/// items of the same type, including alignment padding.
///
/// The stabilized version of this intrinsic is [`core::mem::size_of`].
#[rustc_nounwind]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub const fn size_of<T>() -> usize {
    unreachable!()
}

#[rustc_nounwind]
#[rustc_intrinsic]
#[rustc_intrinsic_must_be_overridden]
pub fn abort() -> ! {
    unreachable!()
}

// extern "rust-intrinsic" {
//     // TODO: rustc_const_stable
//     // #[rustc_const_stable(feature = "const_size_of", since = "1.0.0")]
//     #[rustc_safe_intrinsic]
//     #[rustc_nounwind]
//     pub fn size_of<T>() -> usize;

//     #[rustc_nounwind]
//     pub fn transmute<Src, Dst>(src: Src) -> Dst;

//     #[rustc_nounwind]
//     pub fn transmute_unchecked<Src, Dst>(src: Src) -> Dst;

//     #[rustc_nounwind]
//     pub fn read_via_copy<T>(ptr: *const T) -> T;

//     #[rustc_nounwind]
//     pub fn write_via_move<T>(ptr: *mut T, value: T);
// }
