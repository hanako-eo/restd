use std::cell::RefCell;
use std::fmt::Display;

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Error, Result};

#[derive(Default)]
pub struct Context {
    dirty_errors: RefCell<Vec<String>>,
    errors: RefCell<Vec<Error>>,
}

impl Context {
    pub fn push_error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.push_error(Error::new_spanned(obj.into_token_stream(), msg));
    }

    pub fn push_error(&self, error: Error) {
        self.errors.borrow_mut().push(error);
    }

    pub fn push_dirty_error<D: Display>(&self, dirty_error: D) {
        self.dirty_errors.borrow_mut().push(dirty_error.to_string());
    }

    /// Consume this object, producing a formatted error string if there are errors.
    pub fn check<F, T>(self, merge_dirty: F) -> Result<()>
    where
        T: Display,
        F: FnOnce(Vec<String>) -> Option<T>,
    {
        let mut errors = self.errors.into_inner().into_iter();

        let mut combined = match merge_dirty(self.dirty_errors.into_inner())
            .map(|msg| Error::new(Span::call_site(), msg))
            .or_else(|| errors.next())
        {
            Some(first) => first,
            None => return Ok(()),
        };

        for rest in errors {
            combined.combine(rest);
        }

        Err(combined)
    }
}
