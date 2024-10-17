use crate::core::macros::{
    internal_impl_assign_binop_native, internal_impl_binop_native, internal_impl_unop_native,
};

#[lang = "add"]
#[doc(alias = "+")]
pub trait Add<Rhs = Self> {
    type Output;

    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    fn add(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl Add, add (+) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "sub"]
#[doc(alias = "-")]
pub trait Sub<Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl Sub, sub (-) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "mul"]
#[doc(alias = "*")]
pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    impl Mul, mul (*) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "div"]
#[doc(alias = "/")]
pub trait Div<Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    #[doc = "This operation will panic if `other == 0`."]
    impl Div, div (/) for usize, u8, u16, u32, u64, u128
}

internal_impl_binop_native! {
    #[doc = "This operation will panic if `other == 0` or the division results in overflow."]
    impl Div, div (/) for isize, i8, i16, i32, i64, i128
}

internal_impl_binop_native! {
    impl Div, div (/) for f16, f32, f64, f128
}

#[lang = "rem"]
#[doc(alias = "%")]
pub trait Rem<Rhs = Self> {
    type Output;

    fn rem(self, rhs: Rhs) -> Self::Output;
}

internal_impl_binop_native! {
    #[doc = "This operation will panic if `other == 0`."]
    impl Rem, rem (%) for usize, u8, u16, u32, u64, u128
}

internal_impl_binop_native! {
    #[doc = "This operation will panic if `other == 0` or if `self / other` results in overflow."]
    impl Rem, rem (%) for isize, i8, i16, i32, i64, i128
}

internal_impl_binop_native! {
    impl Rem, rem (%) for f16, f32, f64, f128
}

#[lang = "neg"]
#[doc(alias = "-")]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}

internal_impl_unop_native! { impl Neg, neg (-) for isize, i8, i16, i32, i64, i128, f16, f32, f64, f128 }

#[lang = "add_assign"]
#[doc(alias = "+=")]
pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl AddAssign, add_assign (+=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "sub_assign"]
#[doc(alias = "-=")]
pub trait SubAssign<Rhs = Self> {
    fn sub_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl SubAssign, sub_assign (-=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "mul_assign"]
#[doc(alias = "*=")]
pub trait MulAssign<Rhs = Self> {
    fn mul_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl MulAssign, mul_assign (*=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "div_assign"]
#[doc(alias = "/=")]
pub trait DivAssign<Rhs = Self> {
    fn div_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl DivAssign, div_assign (/=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}

#[lang = "rem_assign"]
#[doc(alias = "%=")]
pub trait RemAssign<Rhs = Self> {
    fn rem_assign(&mut self, rhs: Rhs);
}

internal_impl_assign_binop_native! {
    impl RemAssign, rem_assign (%=) for usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f16, f32, f64, f128
}
