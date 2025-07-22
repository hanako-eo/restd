use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

use crate::internals::Context;
use crate::internals::macros::parsable_attribute;
use crate::internals::symbol::{
    CONSTNESS, FEATURE, INTRINSIC, ISSUE, NOTE, REASON, SINCE, STABILISABLE, SUGGESTION, Symbol,
};

pub struct Attr<'c, T> {
    ctx: &'c Context,
    name: Symbol,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    fn none(ctx: &'c Context, name: Symbol) -> Self {
        Self {
            ctx,
            name,
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            let msg = format!("duplicate attribute `{}`", self.name);
            self.ctx.push_error_spanned_by(tokens, msg);
        } else {
            self.value = Some(value);
        }
    }

    fn get_opt(self) -> Option<T> {
        self.value
    }

    fn get(self) -> T
    where
        T: Default,
    {
        match self.value {
            Some(value) => value,
            None => {
                self.ctx.push_dirty_error(self.name);
                T::default()
            }
        }
    }
}

pub struct BoolAttr<'c>(Attr<'c, bool>);

impl<'c> BoolAttr<'c> {
    fn none(ctx: &'c Context, name: Symbol) -> Self {
        Self(Attr::none(ctx, name))
    }

    fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, true);
    }

    fn set_false<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, true);
    }

    fn get(self) -> bool {
        self.0.get_opt().unwrap_or_default()
    }
}

parsable_attribute! {
    pub struct StableAttribute {
        pub constness: bool = CONSTNESS,

        pub feature: String = FEATURE,
        pub since: String = SINCE,
    }
}

parsable_attribute! {
    pub struct UnstableAttribute {
        pub constness: bool = CONSTNESS,
        pub stabilisable: bool = STABILISABLE,
        pub intrinsic: bool = INTRINSIC,

        pub feature: String = FEATURE,
        pub issue: String = ISSUE,
        pub reason: String = REASON,
    }
}

parsable_attribute! {
    pub struct DeprecatedAttribute {
        pub since: String = SINCE,
        pub note: String = NOTE,
        pub suggestion: String = SUGGESTION,
    }
}

fn get_value<T: Parse>(input: &ParseStream) -> Result<T> {
    input.parse::<Token![=]>()?;
    input.parse::<T>()
}
