use native_core::ops::Deref;

use crate::matches;

use self::Option::*;

pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    #[inline]
    pub fn and<U>(self, other: Option<U>) -> Option<U> {
        match (self, other) {
            (Some(_), Some(value)) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Some(value) => f(value),
            None => None,
        }
    }

    #[inline]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Some(value), _) | (None, Some(value)) => Some(value),
            (None, None) => None,
        }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            Some(value) => Some(value),
            None => f(),
        }
    }

    #[inline]
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Some(value), None) | (None, Some(value)) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }

    #[inline]
    pub fn filter<F>(self, f: F) -> Self
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Some(value) => match f(&value) {
                true => Some(value),
                false => None,
            },
            None => None,
        }
    }

    #[inline]
    pub const fn as_deref(&self) -> Option<&T>
    where
        T: Deref,
    {
        match self {
            Some(value) => Some(value),
            None => None,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> Option<&T> {
        match self {
            Some(value) => Some(value),
            None => None,
        }
    }

    #[inline]
    pub const fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Some(value) => Some(value),
            None => None,
        }
    }

    // TODO
    // #[inline]
    // pub unsafe fn unwrap_unchecked(self) -> T {}

    // const need compiled time Drop, not the case for now
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Some(value) => value,
            None => panic!("no value for `Option::None`"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, otherwise: T) -> T {
        match self {
            Some(value) => value,
            None => otherwise,
        }
    }

    #[inline]
    pub fn unwrap_or_default<F>(self) -> T
    where
        T: Default,
    {
        match self {
            Some(value) => value,
            None => T::default(),
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(value) => value,
            None => f(),
        }
    }

    #[inline]
    pub const fn is_some(&self) -> bool {
        matches!(*self, Some(_))
    }

    #[inline]
    pub fn is_some_and<F>(self, f: F) -> bool
    where
        F: FnOnce(T) -> bool,
    {
        match self {
            Some(value) => f(value),
            None => false,
        }
    }

    #[inline]
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    // TODO: mem::take
    // #[inline]
    // pub fn take(&mut self) -> Option<T> {}

    // TODO: mem::replace
    // #[inline]
    // pub fn replace(&mut self, new_value: T) -> Option<T> {}
}

impl<T: Clone> Clone for Option<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}

impl<T: Clone> Option<&T> {
    #[inline]
    pub fn cloned(self) -> Option<T> {
        match self {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}

impl<T: Copy> Copy for Option<T> {}

impl<T: Copy> Option<&T> {
    #[inline]
    pub fn copied(self) -> Option<T> {
        match self {
            Some(value) => Some(*value),
            None => None,
        }
    }
}

impl<T> Option<Option<T>> {
    #[inline]
    pub fn flatten(self) -> Option<T> {
        self.and_then(|x| x)
    }
}
