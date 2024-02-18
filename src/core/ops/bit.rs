#[lang = "bitand"]
pub trait BitAnd<Rhs = Self> {
    type Output;

    fn bitand(self, rhs: Rhs) -> Self::Output;
}
#[lang = "bitand_assign"]
pub trait BitAndAssign<Rhs = Self> {
    fn bitand_assign(&mut self, rhs: Rhs);
}

#[lang = "bitor"]
pub trait BitOr<Rhs = Self> {
    type Output;

    fn bitor(self, rhs: Rhs) -> Self::Output;
}
#[lang = "bitor_assign"]
pub trait BitOrAssign<Rhs = Self> {
    fn bitor_assign(&mut self, rhs: Rhs);
}

#[lang = "bitxor"]
pub trait BitXor<Rhs = Self> {
    type Output;

    fn bitxor(self, rhs: Rhs) -> Self::Output;
}
#[lang = "bitxor_assign"]
pub trait BitXorAssign<Rhs = Self> {
    fn bitxor_assign(&mut self, rhs: Rhs);
}

#[lang = "shl"]
pub trait Shl<Rhs = Self> {
    type Output;

    fn shl(self, rhs: Rhs) -> Self::Output;
}
#[lang = "shl_assign"]
pub trait ShlAssign<Rhs = Self> {
    fn shl_assign(&mut self, rhs: Rhs);
}

#[lang = "shr"]
pub trait Shr<Rhs = Self> {
    type Output;

    fn shr(self, rhs: Rhs) -> Self::Output;
}
#[lang = "shr_assign"]
pub trait ShrAssign<Rhs = Self> {
    fn shr_assign(&mut self, rhs: Rhs);
}

#[lang = "not"]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
