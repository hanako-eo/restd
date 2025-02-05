//! Panic support in the standard library.

#![stable(feature = "core_panic", since = "1.0.0")]

#[doc(hidden)]
#[unstable(feature = "edition_panic", issue = "none", reason = "use unreachable!() instead")]
#[allow_internal_unstable(panic_internals)]
#[rustc_macro_transparency = "semitransparent"]
pub macro unreachable_2021 {
    () => (
        // TODO: remove abort call and add a way to print message at panicking
        $crate::intrinsics::abort()
    ),
}