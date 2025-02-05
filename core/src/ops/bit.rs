use crate::core::macros::{
    internal_impl_assign_binop_native, internal_impl_binop_native, internal_impl_unop_native,
};

#[lang = "bitand"]
#[doc(alias = "&")]
pub trait BitAnd<Rhs = Self> {
    type Output;

    fn bitand(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl BitAnd, bitand (&) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

#[lang = "bitor"]
#[doc(alias = "|")]
pub trait BitOr<Rhs = Self> {
    type Output;

    fn bitor(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl BitOr, bitor (|) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

#[lang = "bitxor"]
#[doc(alias = "^")]
pub trait BitXor<Rhs = Self> {
    type Output;

    fn bitxor(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl BitXor, bitxor (^) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

macro_rules! impl_shifting {
    (impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {
        internal_impl_binop_native! { impl $trait<usize>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<u8>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<u16>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<u32>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<u64>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<u128>, $method ($symbol) for $($t),* }

        internal_impl_binop_native! { impl $trait<isize>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<i8>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<i16>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<i32>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<i64>, $method ($symbol) for $($t),* }
        internal_impl_binop_native! { impl $trait<i128>, $method ($symbol) for $($t),* }
    };
}

#[lang = "shl"]
#[doc(alias = "<<")]
pub trait Shl<Rhs = Self> {
    type Output;

    fn shl(self, rhs: Rhs) -> Self::Output;
}

impl_shifting! { impl Shl, shl (<<) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128 }

#[lang = "shr"]
#[doc(alias = ">>")]
pub trait Shr<Rhs = Self> {
    type Output;

    fn shr(self, rhs: Rhs) -> Self::Output;
}

impl_shifting! { impl Shr, shr (>>) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128 }

#[lang = "not"]
#[doc(alias = "!")]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}

impl Not for ! {
    type Output = !;

    fn not(self) -> Self::Output {
        self
    }
}

internal_impl_unop_native! { impl Not, not (!) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128 }

#[lang = "bitand_assign"]
#[doc(alias = "&=")]
pub trait BitAndAssign<Rhs = Self> {
    fn bitand_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl BitAndAssign, bitand_assign (&=) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

#[lang = "bitor_assign"]
#[doc(alias = "|=")]
pub trait BitOrAssign<Rhs = Self> {
    fn bitor_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl BitOrAssign, bitor_assign (|=) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

#[lang = "bitxor_assign"]
#[doc(alias = "^=")]
pub trait BitXorAssign<Rhs = Self> {
    fn bitxor_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl BitXorAssign, bitxor_assign (^=) for bool, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
}

macro_rules! impl_shifting_assign {
    (impl $trait:ident, $method:ident ($symbol:tt) for $($t:ty),+) => {
        internal_impl_assign_binop_native! { impl $trait<usize>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<u8>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<u16>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<u32>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<u64>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<u128>, $method ($symbol) for $($t),* }

        internal_impl_assign_binop_native! { impl $trait<isize>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<i8>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<i16>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<i32>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<i64>, $method ($symbol) for $($t),* }
        internal_impl_assign_binop_native! { impl $trait<i128>, $method ($symbol) for $($t),* }
    };
}

#[lang = "shl_assign"]
#[doc(alias = "<<=")]
pub trait ShlAssign<Rhs = Self> {
    fn shl_assign(&mut self, rhs: Rhs);
}

impl_shifting_assign! { impl ShlAssign, shl_assign (<<=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128 }

#[lang = "shr_assign"]
#[doc(alias = ">>=")]
pub trait ShrAssign<Rhs = Self> {
    fn shr_assign(&mut self, rhs: Rhs);
}

impl_shifting_assign! { impl ShrAssign, shr_assign (>>=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128 }
