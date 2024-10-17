#![macro_use]

#[macro_export]
macro_rules! matches {
    ($cond:expr, $pat:pat $(if $guard:expr)? $(,)?) => {
        match $cond {
            $pat $(if $guard)? => true,
            _ => false
        }
    };
}

/// This macro implement the version of "&T" for unary operators
macro_rules! forward_ref_unop {
    (impl $trait:ident, $method:ident for $t:ty) => {
        impl $trait for &$t {
            type Output = <$t as $trait>::Output;

            #[inline]
            fn $method(self) -> <$t as $trait>::Output {
                $trait::$method(*self)
            }
        }
    };
}

/// This macro implement the version of "&U for T", "U for &T" and "&U for &T" for binary operators
macro_rules! forward_ref_binop {
    (impl $trait:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $trait<$u> for &'a $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, other)
            }
        }

        impl $trait<&$u> for $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $trait<$u>>::Output {
                $trait::$method(self, *other)
            }
        }

        impl $trait<&$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, *other)
            }
        }
    };
}

/// This macro implement the version of "&U for T" for assignment operators
macro_rules! forward_ref_op_assign {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}

/// This macro is designed for implement traits like Neg
macro_rules! internal_impl_unop_native {
    (impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        impl $trait for $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            fn $method(self) -> Self::Output {
                $symbol self
            }
        }

        forward_ref_unop! { impl $trait, $method for $t }
    )+};
    (#[$meta:meta] impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        #[$meta]
        internal_impl_unop_native! { impl $trait, $method ($symbol) for $t }
    )+};
}

/// This macro is designed for implement traits like Add or Sub
macro_rules! internal_impl_binop_native {
    (impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        internal_impl_binop_native! { impl $trait<$t>, $method ($symbol) for $t }
    )+};
    (impl $trait:ident <$f:ty>, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        impl $trait<$f> for $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            #[rustc_inherit_overflow_checks]
            fn $method(self, other: $f) -> Self::Output {
                self $symbol other
            }
        }

        forward_ref_binop! { impl $trait, $method for $t, $f }
    )+};
    (#[$meta:meta] impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        #[$meta]
        internal_impl_binop_native! { impl $trait, $method ($symbol) for $t }
    )+};
}

/// This macro is designed for implement traits like Add or Sub
macro_rules! internal_impl_assign_binop_native {
    (impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        internal_impl_assign_binop_native! { impl $trait<$t>, $method ($symbol) for $t }
    )+};
    (impl $trait:ident <$f:ty>, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        impl $trait<$f> for $t {
            #[inline]
            #[track_caller]
            #[rustc_inherit_overflow_checks]
            fn $method(&mut self, other: $f) {
                *self $symbol other;
            }
        }

        forward_ref_op_assign! { impl $trait, $method for $t, $f }
    )+};
    (#[$meta:meta] impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {$(
        #[$meta]
        internal_impl_assign_binop_native! { impl $trait, $method ($symbol) for $t }
    )+};
}

// This trick allow the usage of the macros exported without the inconvence of
// the #[macro_export] that is more like an pub
pub(crate) use {
    internal_impl_assign_binop_native, internal_impl_binop_native, internal_impl_unop_native,
};
