use native_core::{fmt::Debug, ops::Deref};

use crate::matches;

use self::Result::*;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    #[inline]
    pub fn and<U>(self, other: Result<U, E>) -> Result<U, E> {
        match (self, other) {
            (Ok(_), Ok(value)) => Ok(value),
            (Err(err), _) | (_, Err(err)) => Err(err),
        }
    }

    #[inline]
    pub fn and_then<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Ok(value) => f(value),
            Err(err) => Err(err),
        }
    }

    #[inline]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Ok(value), _) | (_, Ok(value)) => Ok(value),
            (Err(_), Err(err)) => Err(err),
        }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce(E) -> Result<T, E>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => f(err),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(value) => Ok(f(value)),
            Err(err) => Err(err),
        }
    }

    #[inline]
    pub fn map_err<U, F>(self, f: F) -> Result<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(f(err)),
        }
    }

    #[inline]
    pub const fn as_deref(&self) -> Result<&T, &E>
    where
        T: Deref,
        E: Deref,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> Result<&T, &E> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    #[inline]
    pub const fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    // TODO
    // #[inline]
    // pub unsafe fn unwrap_unchecked(self) -> T {}

    #[inline]
    pub fn unwrap(self) -> T
    where
        E: Debug,
    {
        match self {
            Ok(value) => value,
            Err(err) => panic!("no value for `Result::Err({:?})`", err),
        }
    }

    #[inline]
    pub fn unwrap_or(self, otherwise: T) -> T {
        match self {
            Ok(value) => value,
            Err(_) => otherwise,
        }
    }

    #[inline]
    pub fn unwrap_or_default<F>(self) -> T
    where
        T: Default,
    {
        match self {
            Ok(value) => value,
            Err(_) => T::default(),
        }
    }

    #[inline]
    pub fn unwrap_err(self, otherwise: E) -> E {
        match self {
            Ok(_) => otherwise,
            Err(err) => err,
        }
    }

    #[inline]
    pub fn unwrap_err_default<F>(self) -> E
    where
        E: Default,
    {
        match self {
            Ok(_) => E::default(),
            Err(err) => err,
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Ok(value) => value,
            Err(err) => f(err),
        }
    }

    #[inline]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, Ok(_))
    }

    #[inline]
    pub fn is_ok_and<F>(self, f: F) -> bool
    where
        F: FnOnce(T) -> bool,
    {
        match self {
            Ok(value) => f(value),
            Err(_) => false,
        }
    }

    #[inline]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn is_err_and<F>(self, f: F) -> bool
    where
        F: FnOnce(E) -> bool,
    {
        match self {
            Ok(_) => false,
            Err(err) => f(err),
        }
    }

    // TODO: mem::take
    // #[inline]
    // pub fn take(&mut self) -> Result<T, E> {}

    // TODO: mem::replace
    // #[inline]
    // pub fn replace(&mut self, new_value: T) -> Result<T, E> {}
}

impl<T: Clone, E: Clone> Clone for Result<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<T: Clone, E: Clone> Result<&T, &E> {
    #[inline]
    pub fn cloned(self) -> Result<T, E> {
        match self {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<T: Copy, E: Copy> Copy for Result<T, E> {}

impl<T: Copy, E: Copy> Result<&T, &E> {
    #[inline]
    pub fn copied(self) -> Result<T, E> {
        match self {
            Ok(value) => Ok(*value),
            Err(err) => Err(*err),
        }
    }
}

impl<T, E> Result<Result<T, E>, E> {
    #[inline]
    pub fn flatten(self) -> Result<T, E> {
        self.and_then(|x| x)
    }
}
