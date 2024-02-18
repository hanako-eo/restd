#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
#[lang = "add_assign"]
pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

#[lang = "sub"]
pub trait Sub<Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs) -> Self::Output;
}
#[lang = "sub_assign"]
pub trait SubAssign<Rhs = Self> {
    fn sub_assign(&mut self, rhs: Rhs);
}

#[lang = "mul"]
pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs) -> Self::Output;
}
#[lang = "mul_assign"]
pub trait MulAssign<Rhs = Self> {
    fn mul_assign(&mut self, rhs: Rhs);
}

#[lang = "div"]
pub trait Div<Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs) -> Self::Output;
}
#[lang = "div_assign"]
pub trait DivAssign<Rhs = Self> {
    fn div_assign(&mut self, rhs: Rhs);
}

#[lang = "rem"]
pub trait Rem<Rhs = Self> {
    type Output;

    fn rem(self, rhs: Rhs) -> Self::Output;
}
#[lang = "rem_assign"]
pub trait RemAssign<Rhs = Self> {
    fn rem_assign(&mut self, rhs: Rhs);
}

#[lang = "neg"]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}
