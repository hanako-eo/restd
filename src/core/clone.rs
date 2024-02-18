use super::marker::Sized;

#[lang = "clone"]
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

macro_rules! impl_clone {
    ($($t:ty,)*) => {
        $(
            impl Clone for $t {
                #[inline(always)]
                fn clone(&self) -> Self {
                    *self
                }
            }
        )*
    };
    ($g:ident : $($t:ty,)*) => {
        $(
            impl<$g> Clone for $t {
                #[inline(always)]
                fn clone(&self) -> Self {
                    *self
                }
            }
        )*
    };
}

impl_clone! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    bool, char,
}
impl_clone! {
    T: *mut T,
    *const T,
    &T,
}

impl Clone for ! {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> !Clone for &mut T {}
