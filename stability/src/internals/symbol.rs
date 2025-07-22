use std::fmt::{self, Display};

use syn::{Ident, Path};

pub const CONSTNESS: Symbol = Symbol("constness");

// stable macro symbols
pub const FEATURE: Symbol = Symbol("feature");
pub const SINCE: Symbol = Symbol("since");

// unstable macro symbols
pub const ISSUE: Symbol = Symbol("issue");
pub const REASON: Symbol = Symbol("reason");
// will add #[rustc_const_stable_indirect]
pub const STABILISABLE: Symbol = Symbol("stabilisable");
pub const INTRINSIC: Symbol = Symbol("intrinsic");

// deprecated macro symbols
pub const NOTE: Symbol = Symbol("note");
pub const SUGGESTION: Symbol = Symbol("suggestion");

#[derive(Debug, Clone, Copy)]
pub struct Symbol(pub &'static str);

impl PartialEq<Ident> for Symbol {
    fn eq(&self, word: &Ident) -> bool {
        word == self.0
    }
}

impl PartialEq<Path> for Symbol {
    fn eq(&self, word: &Path) -> bool {
        word.is_ident(self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
