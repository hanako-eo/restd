// based on the corresponding file of the rust core:
// https://github.com/rust-lang/rust/blob/master/library/core/src/prelude/mod.rs

//! The core prelude of restd

// No formatting: this file is nothing but re-exports, and their order is worth preserving.
#![cfg_attr(rustfmt, rustfmt::skip)]

#![stable(feature = "core_prelude", since = "1.0.0")]

/// The first version of the prelude of The Rust Standard Library.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_common", since = "1.0.0")]
pub mod common {
    #[stable(feature = "prelude_common", since = "1.0.0")]
    #[doc(no_inline)]
    pub use crate::unreachable;
}

/// The 2015 version of the core prelude.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2015", since = "1.0.0")]
pub mod rust_2015 {
    #[stable(feature = "prelude_2015", since = "1.0.0")]
    #[doc(no_inline)]
    pub use super::common::*;
}

/// The 2018 version of the core prelude.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2018", since = "1.0.0")]
pub mod rust_2018 {
    #[stable(feature = "prelude_2018", since = "1.0.0")]
    #[doc(no_inline)]
    pub use super::common::*;
}

/// The 2021 version of the core prelude.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2021", since = "1.0.0")]
pub mod rust_2021 {
    #[stable(feature = "prelude_2021", since = "1.0.0")]
    #[doc(no_inline)]
    pub use super::common::*;
}

/// The 2024 version of the core prelude.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2024", since = "1.0.0")]
pub mod rust_2024 {
    #[stable(feature = "prelude_common", since = "1.0.0")]
    pub use super::common::*;
}
