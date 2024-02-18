extern "rust-intrinsic" {
    // TODO: rustc_const_stable
    // #[rustc_const_stable(feature = "const_size_of", since = "1.0.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn size_of<T>() -> usize;

    #[rustc_nounwind]
    pub fn transmute<Src, Dst>(src: Src) -> Dst;

    #[rustc_nounwind]
    pub fn transmute_unchecked<Src, Dst>(src: Src) -> Dst;

    #[rustc_nounwind]
    pub fn read_via_copy<T>(ptr: *const T) -> T;

    #[rustc_nounwind]
    pub fn write_via_move<T>(ptr: *mut T, value: T);
}
