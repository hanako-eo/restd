use quote::{ToTokens, quote};

use crate::symbol::{REPR, Symbol, TRANSPARENT};
use crate::{const_trait_symbols, fundamental_symbols};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemAttribut {
    Unknown,

    ConstTrait,
    Fundamental,
    Transparent,
}

impl From<syn::Meta> for ItemAttribut {
    fn from(meta: syn::Meta) -> Self {
        let containt_at_least_one_of =
            |path: &syn::Path, symbols: &[Symbol]| symbols.iter().any(|symbol| symbol == path);

        match meta {
            syn::Meta::Path(path) if containt_at_least_one_of(&path, &const_trait_symbols) => {
                Self::ConstTrait
            }
            syn::Meta::Path(path) if containt_at_least_one_of(&path, &fundamental_symbols) => {
                Self::Fundamental
            }
            syn::Meta::List(list) if REPR == list.path => {
                // SAFETY: repr cannot take anything else but ident
                let ident = unsafe { syn::parse2::<syn::Ident>(list.tokens).unwrap_unchecked() };
                if TRANSPARENT == ident {
                    Self::Transparent
                } else {
                    Self::Unknown
                }
            }
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Enum,
    Trait,
    Struct,
    Union,
    Fn,
}

impl Target {
    pub fn name(self) -> &'static str {
        match self {
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Struct => "struct",
            Self::Union => "union",
            Self::Fn => "function",
        }
    }
}

#[allow(unused_imports)]
pub(crate) use Constraint::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Constraint {
    // Traits constraints
    Auto,
    Unsafety,
    Constness,

    // Struct/Union constraints
    Transparent,
    Fundamental,

    Generics(usize),
    Arguments(usize),
}

impl Constraint {
    pub fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Unsafety => "unsafe",
            Self::Constness => "const",
            Self::Transparent => "transparent",
            Self::Fundamental => "fundamental",
            Self::Generics(_) => "generics",
            Self::Arguments(_) => "arguments",
        }
    }
}

macro_rules! lang_item_table {
    ($($(#[$attr:meta])* $variant:ident, $name:literal, $target:expr, [$($constraint:expr),*], $diag_item:literal, $lang_item:literal;)*) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum LangItem {
            $($(#[$attr])* $variant,)*
        }

        impl TryFrom<syn::LitStr> for LangItem {
            type Error = syn::Error;

            fn try_from(value: syn::LitStr) -> Result<Self, Self::Error> {
                match value.value().as_str() {
                    $($(#[$attr])* $name => Ok(Self::$variant),)*
                    lit => Err(syn::Error::new_spanned(value, format!("\"{lit}\" is not a valid compiler lang item."))),
                }
            }
        }

        impl ToTokens for LangItem {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {$(
                    $(#[$attr])*
                    Self::$variant => {
                        let diag_item = $diag_item.then(|| {
                            let variant = stringify!($variant);
                            quote! { #[rustc_diagnostic_item = #variant] }
                        });
                        let lang_item = $lang_item.then(|| {
                            quote! { #[lang = $name] }
                        });
                        *tokens = quote! {
                            #diag_item
                            #lang_item
                        };
                    },
                )*}
            }
        }

        impl LangItem {
            pub fn name(self) -> &'static str {
                match self {$(
                    $(#[$attr])*
                    Self::$variant => $name,
                )*}
            }

            pub fn target(self) -> Target {
                match self {$(
                    $(#[$attr])*
                    Self::$variant => $target,
                )*}
            }

            pub fn constraints(self) -> &'static [Constraint] {
                match self {$(
                    $(#[$attr])*
                    Self::$variant => &[$($constraint),*],
                )*}
            }
        }
    };
}

lang_item_table! {
    //  Variant name,    Item name,    Item target,      Contains,    Is diag item,    Is lang item
    Sized,           "sized",      Target::Trait,    [],          false,           true;
    Unsize,          "unsize",     Target::Trait,    [],          false,           true;
    Copy,            "copy",       Target::Trait,    [],          false,           true;
    Clone,           "clone",      Target::Trait,    [],          false,           true;
    Drop,            "drop",       Target::Trait,    [],          false,           true;

    // TEST ITEMS
    #[cfg(feature = "test_lang_item")]
    TestAuto,        "foo_auto",      Target::Trait,    [Auto],         false, false;
    #[cfg(feature = "test_lang_item")]
    TestConstness,   "foo_const",     Target::Trait,    [Constness],    false, false;
    #[cfg(feature = "test_lang_item")]
    TestUnsafety,    "foo_unsafe",    Target::Trait,    [Unsafety],     false, false;
    #[cfg(feature = "test_lang_item")]
    TestFnUnsafety,  "foo_fn_unsafe", Target::Fn,       [Unsafety],     false, false;
    #[cfg(feature = "test_lang_item")]
    TestFn,          "foo_fn",        Target::Fn,       [Arguments(1)], false, false;
}

// Add
// Sub
// Mul
// Div
// Rem
// Neg
// Not
// BitXor
// BitAnd
// BitOr
// Shl
// Shr
// AddAssign
// SubAssign
// MulAssign
// DivAssign
// RemAssign
// BitXorAssign
// BitAndAssign
// BitOrAssign
// ShlAssign
// ShrAssign
// Index
// IndexMut
// PartialEq
// PartialOrd
//
// RangeFrom
// RangeFull
// RangeInclusiveStruct
// RangeInclusiveNew
// Range
// RangeToInclusive
// RangeTo
//
// Deref
// DerefMut
// DerefPure
// DerefTarget
// Receiver
// ReceiverTarget
// LegacyReceiver
//
// Fn
// FnMut
// FnOnce
//
// Iterator
// IteratorNext
//
// Unpin
// Pin
//
// PhantomData
// ManuallyDrop
