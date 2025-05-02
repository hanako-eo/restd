use quote::{ToTokens, quote};

use crate::symbol::{C, REPR, Symbol, TRANSPARENT};
use crate::{
    coinductive_symbols, const_trait_symbols, fundamental_symbols, object_unimplementable_symbols,
    unimplementable_symbols,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemAttribut {
    Unknown,

    ConstTrait,
    Fundamental,
    Transparent,
    CLayout,
    Coinductive,
    Unimplementable,
    ObjectUnimplementable,
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
            syn::Meta::Path(path) if containt_at_least_one_of(&path, &coinductive_symbols) => {
                Self::Coinductive
            }
            syn::Meta::Path(path)
                if containt_at_least_one_of(&path, &object_unimplementable_symbols) =>
            {
                Self::Unimplementable
            }
            syn::Meta::Path(path) if containt_at_least_one_of(&path, &unimplementable_symbols) => {
                Self::Unimplementable
            }
            syn::Meta::List(list) if REPR == list.path => {
                // SAFETY: repr cannot take anything else but ident
                let ident = unsafe { syn::parse2::<syn::Ident>(list.tokens).unwrap_unchecked() };
                if TRANSPARENT == ident {
                    Self::Transparent
                } else if C == ident {
                    Self::CLayout
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

    Type,
    EnumVariant,
}

impl Target {
    pub fn name(self) -> &'static str {
        match self {
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Struct => "struct",
            Self::Union => "union",
            Self::Fn => "function",
            Self::Type => "type",
            Self::EnumVariant => "enum variant",
        }
    }
}

pub(crate) use Constraint::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Constraint {
    // Traits constraints
    Auto,
    Unsafety,
    Constness,
    Coinductive,
    Unimplementable,
    ObjectUnimplementable,

    // Struct/Union constraints
    CLayout,
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
            Self::Coinductive => "coinductive",
            Self::Unimplementable => "unimplementable",
            Self::ObjectUnimplementable => "object unimplementable",
            Self::CLayout => "repr(C)",
            Self::Transparent => "repr(transparent)",
            Self::Fundamental => "fundamental",
            Self::Generics(_) => "generics",
            Self::Arguments(_) => "arguments",
        }
    }
}

use DialogItem::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DialogItem {
    /// Overrides the name of the item (depending on whether it's a dialogue or
    /// a lang item) for the codegen.
    Named(&'static str),
    /// Used to take the name of the internal item as the name given to the
    /// compiler.
    Inherited,
    /// Does not generate the dialog or lang item attribute.
    No,
}

macro_rules! lang_item_table {
    ($($(#[$attr:meta])* $variant:ident, $name:literal, $target:expr, [$($constraint:expr),*], $diag_item:expr, $lang_item:expr;)*) => {
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
                        let diag_item = match $diag_item {
                        	Named(name) => Some(quote! { #[rustc_diagnostic_item = #name] }),
                         	Inherited => {
                              let variant = stringify!($variant);
                              Some(quote! { #[rustc_diagnostic_item = #variant] })
                          	},
                        	No => None,
                        };
                        let lang_item = match $lang_item {
                        	Named(name) => Some(quote! { #[rustc_diagnostic_item = #name] }),
                         	Inherited => Some(quote! { #[rustc_diagnostic_item = $name] }),
                        	No => None,
                        };
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

// TODO: add items related on Ranges and all #[rustc_diagnostic_item]
lang_item_table! {
    // Macro format:
    // Variant name, Item name, Item target, Constraints, Is diag item, Is lang item

    // Lang item based on trait and forced by the compiler to have a custom behavior.
    Sized, "sized", Target::Trait, [Coinductive, Fundamental, Unimplementable], No, Inherited;
    Unsize, "unsize", Target::Trait, [Generics(1), Unimplementable], No, Inherited;
    CoerceUnsized, "coerce_unsized", Target::Trait, [], No, Inherited;
    ConstParamTy, "const_param_ty", Target::Trait, [], No, Inherited;
    UnsizedConstParamTy, "unsized_const_param_ty", Target::Trait, [], No, Inherited;

    TupleTrait, "tuple_trait", Target::Trait, [Unimplementable], No, Inherited;
    StructuralPartialEq, "structural_peq", Target::Trait, [], No, Inherited;

    DiscriminantKind, "discriminant_kind", Target::Trait, [Unimplementable], No, Inherited;
    DiscriminantType, "discriminant_type", Target::Type, [], No, Inherited;

    Send, "send", Target::Trait, [Unsafety, Auto], No, No;
    Sync, "sync", Target::Trait, [Unsafety, Auto], No, Inherited;
    Freeze, "freeze", Target::Trait, [Unsafety, Auto], No, Inherited;
    Unpin, "unpin", Target::Trait, [Auto], No, Inherited;

    // TODO: keep track of use_cloned
    // https://github.com/rust-lang/rust/issues/132290
    Copy, "copy", Target::Trait, [], Inherited, Inherited;
    Clone, "clone", Target::Trait, [], Inherited, Inherited;
    CloneFn, "clone_fn", Target::Fn, [Arguments(1)], No, Inherited;

    Drop, "drop", Target::Trait, [Constness], No, Inherited;
    Destruct, "destruct", Target::Trait, [Constness, Unimplementable], No, Inherited;
    BikeshedGuaranteedNoDrop, "bikeshed_guaranteed_no_drop", Target::Trait, [Unimplementable], No, Inherited;

    AsyncDrop, "async_drop", Target::Trait, [], No, Inherited;
    FutureTrait, "future_trait", Target::Trait, [], No, Inherited;
    FutureOutput, "future_output", Target::Type, [], No, Inherited;
    FuturePoll, "future_poll", Target::Fn, [Arguments(2)], No, Named("poll");
    IntoFutureTrait, "into_future_trait", Target::Trait, [], Named("IntoFuture"), No;
    IntoFutureFn, "into_future_fn", Target::Fn, [Arguments(1)], No, Named("into_future");

    IntoIterTrait, "into_iterator_trait", Target::Trait, [], Named("IntoIterator"), No;
    IntoIterFn, "into_iterator_fn", Target::Fn, [Arguments(1)], No, Named("into_iter");
    FusedIterator, "fused_iterator", Target::Trait, [], No, Inherited;
    Iterator, "iterator", Target::Trait, [], Inherited, Inherited;
    IteratorItem, "iterator_item", Target::Type, [], Inherited, No;
    IteratorNext, "iterator_next", Target::Fn, [Arguments(1)], No, Inherited;

    TransmuteTrait, "transmute_trait", Target::Trait, [Generics(2), Unsafety, Coinductive, Unimplementable], No, Inherited;
    TransmuteAssumption, "transmute_assumption", Target::Struct, [], No, Named("transmute_opts");

    Deref, "deref", Target::Trait, [Constness], Inherited, Inherited;
    DerefMut, "deref_mut", Target::Trait, [Constness], Inherited, Inherited;
    DerefPure, "deref_pure", Target::Trait, [Unsafety], No, Inherited;
    DerefTarget, "deref_target", Target::Type, [], No, Inherited;
    LegacyReceiver, "legacy_receiver", Target::Trait, [], No, Inherited;
    Receiver, "receiver", Target::Trait, [], No, Inherited;
    ReceiverTarget, "receiver_target", Target::Type, [], Named("receiver_target"), Inherited;

    DispatchFromDyn, "dispatch_from_dyn", Target::Trait, [Generics(1)], No, Inherited;

    DynMetadata, "dyn_metadata", Target::Struct, [Generics(1)], No, Inherited;
    PointeeTrait, "pointee", Target::Trait, [Unimplementable], No, Named("pointee_trait");
    PointeeMetadata, "pointee_metadata", Target::Type, [], No, Named("metadata_type");

    PointerLike, "pointer_like", Target::Trait, [ObjectUnimplementable], No, Inherited;
    CoercePointeeValidated, "coerce_pointee_validated", Target::Trait, [], No, Inherited;
    FnPtrTrait, "fn_ptr_trait", Target::Trait, [Unimplementable], No, Inherited;
    FnPtrAddr, "fn_ptr_addr", Target::Fn, [Arguments(1)], No, Inherited;

    AsyncFn, "async_fn", Target::Trait, [Generics(1)], No, Inherited;
    AsyncFnMut, "async_fn_mut", Target::Trait, [Generics(1)], No, Inherited;
    AsyncFnCallRefFuture, "async_fn_call_ref_future", Target::Type, [Generics(1) /* lifetime */], No, Named("call_ref_future");
    AsyncFnOnce, "async_fn_once", Target::Trait, [Generics(1)], No, Inherited;
    AsyncFnCallOnceFuture, "async_fn_call_once_future", Target::Type, [Generics(1) /* lifetime */], No, Named("call_once_future");
    AsyncFnOnceOutput, "async_fn_once_output", Target::Type, [], No, Inherited;
    AsyncFnKindHelper, "async_fn_kind_helper", Target::Trait, [Generics(1)], No, Inherited;
    AsyncFnKindUpvars, "async_fn_kind_upvars", Target::Type, [Generics(4) /* 1 lifetime and 3 generics */], No, Inherited;

    Fn, "fn", Target::Trait, [Generics(1), Fundamental], No, Inherited;
    FnMut, "fn_mut", Target::Trait, [Generics(1), Fundamental], No, Inherited;
    FnOnce, "fn_once", Target::Trait, [Generics(1), Fundamental], No, Inherited;
    FnOnceOutput, "fn_once_output", Target::Type, [], No, Inherited;

    Try, "try", Target::Trait, [], No, Named("Try");
    TryBranchFn, "try_branch_fn", Target::Fn, [Arguments(1)], No, Named("branch");
    TryFromOutputFn, "try_from_output_fn", Target::Fn, [Arguments(1)], No, Named("from_output");
    FromResidual, "from_residual", Target::Trait, [Generics(1)], Inherited, No;
    FromResidualFn, "from_residual_fn", Target::Trait, [], No, Named("from_residual");
    FromYeet, "from_yeet", Target::Fn, [Arguments(1)], No, Inherited;

    Add, "add", Target::Trait, [Generics(1)], No, Inherited;
    Sub, "sub", Target::Trait, [Generics(1)], No, Inherited;
    Mul, "mul", Target::Trait, [Generics(1)], No, Inherited;
    Div, "div", Target::Trait, [Generics(1)], No, Inherited;
    Rem, "rem", Target::Trait, [Generics(1)], No, Inherited;
    BitXor, "bit_xor", Target::Trait, [Generics(1)], No, Inherited;
    BitAnd, "bit_and", Target::Trait, [Generics(1)], No, Inherited;
    BitOr, "bit_or", Target::Trait, [Generics(1)], No, Inherited;
    Shl, "shl", Target::Trait, [Generics(1)], No, Inherited;
    Shr, "shr", Target::Trait, [Generics(1)], No, Inherited;
    AddAssign, "add_assign", Target::Trait, [Generics(1)], No, Inherited;
    SubAssign, "sub_assign", Target::Trait, [Generics(1)], No, Inherited;
    MulAssign, "mul_assign", Target::Trait, [Generics(1)], No, Inherited;
    DivAssign, "div_assign", Target::Trait, [Generics(1)], No, Inherited;
    RemAssign, "rem_assign", Target::Trait, [Generics(1)], No, Inherited;
    BitXorAssign, "bit_xor_assign", Target::Trait, [Generics(1)], No, Inherited;
    BitAndAssign, "bit_and_assign", Target::Trait, [Generics(1)], No, Inherited;
    BitOrAssign, "bit_or_assign", Target::Trait, [Generics(1)], No, Inherited;
    ShlAssign, "shl_assign", Target::Trait, [Generics(1)], No, Inherited;
    ShrAssign, "shr_assign", Target::Trait, [Generics(1)], No, Inherited;
    Index, "index", Target::Trait, [Generics(1)], No, Inherited;
    IndexMut, "index_mut", Target::Trait, [Generics(1)], No, Inherited;
    PartialEq, "partial_eq", Target::Trait, [Generics(1)], Inherited, Named("eq");
    PartialOrd, "partial_ord", Target::Trait, [Generics(1)], Inherited, Inherited;
    Neg, "neg", Target::Trait, [], No, Inherited;
    Not, "not", Target::Trait, [], No, Inherited;

    // Lang item not based on traits.
    ManuallyDrop, "manually_drop", Target::Struct, [Generics(1), Transparent], No, Inherited;
    MaybeUninit, "maybe_uninit", Target::Union, [Generics(1), Transparent], No, Inherited;
    UnsafeCell, "unsafe_cell", Target::Struct, [Generics(1), Transparent], No, Inherited;
    PhantomData, "phantom_data", Target::Struct, [Generics(1)], No, Inherited;
    Ordering, "ordering", Target::Enum, [], No, Named("Ordering");

    ResumeTy, "resume", Target::Struct, [], No, Named("ResumeTy");
    Context, "context", Target::Struct, [Generics(1) /* lifetime */], No, Named("Context");
    GetContext, "get_context", Target::Fn, [Generics(2) /* lifetimes */, Arguments(1), Unsafety], No, Named("get_context");

    AsyncDropInPlace, "async_drop_in_place", Target::Fn, [Generics(1), Arguments(1), Unsafety], Named("ptr_drop_in_place"), Inherited;
    DropInPlace, "drop_in_place", Target::Fn, [Generics(1), Arguments(1), Unsafety], Named("ptr_drop_in_place"), Inherited;

    AllocLayout, "layout", Target::Struct, [], No, Named("alloc_layout");
    Unique, "unique", Target::Struct, [Generics(1), Transparent], No, Named("ptr_unique");
    Pin, "pin", Target::Struct, [Generics(1), Fundamental, Transparent], No, Inherited;
    PinNewUnchecked, "pin_new_unchecked", Target::Struct, [Constness, Unsafety], No, Named("new_unchecked");

    Option, "option", Target::Enum, [Generics(1)], Inherited, Named("Option");
    OptionNone, "option_none", Target::EnumVariant, [], No, Named("None");
    OptionSome, "option_some", Target::EnumVariant, [], No, Named("Some");

    Result, "result", Target::Enum, [Generics(2)], Inherited, Named("Result");
    ResultOk, "result_ok", Target::EnumVariant, [], No, Named("Ok");
    ResultErr, "result_err", Target::EnumVariant, [], No, Named("Err");

    ControlFlow, "control_flow", Target::Enum, [], Inherited, No;
    ControlFlowBreak, "control_flow_break", Target::EnumVariant, [], No, Named("Break");
    ControlFlowContinue, "control_flow_continue", Target::EnumVariant, [], No, Named("Continue");

    Poll, "poll", Target::Enum, [Generics(1)], No, Named("Poll");
    PollReady, "poll_ready", Target::EnumVariant, [], No, Named("Ready");
    PollPending, "poll_pending", Target::EnumVariant, [], No, Named("Pending");

    CoroutineState, "coroutine_state", Target::Enum, [], No, Inherited;
    CoroutineTrait, "coroutine_trait", Target::Trait, [Generics(1), Fundamental], No, Named("coroutine");
    CoroutineYield, "coroutine_yield", Target::Type, [], No, Inherited;
    CoroutineReturn, "coroutine_return", Target::Type, [], No, Inherited;
    CoroutineResume, "coroutine_resume", Target::Fn, [Arguments(2)], No, Inherited;

    SliceLen, "slice_len_fn", Target::Fn, [Arguments(1)], No, Inherited;

    // contracts
    ContractBuildCheckEnsures, "contract_build_check_ensures", Target::Fn, [Generics(2), Arguments(1), Constness], No, Inherited;
    ContractCheckRequires, "contract_check_requires", Target::Fn, [Generics(1), Arguments(1), Constness], No, Inherited;
    ContractCheckEnsures, "contract_check_ensures", Target::Fn, [Generics(2), Arguments(2), Constness], No, Inherited;

    // formating
    FormatArguments, "format_arguments", Target::Struct, [Generics(1) /* lifetime */], No, Inherited;
    FormatArgument, "format_argument", Target::Struct, [Generics(1) /* lifetime */], No, Inherited;
    FormatUnsafeArgument, "format_unsafe_arg", Target::Struct, [], No, Inherited;
    FormatPlaceholder, "format_placeholder", Target::Struct, [Generics(1) /* lifetime */], No, Inherited;
    FormatCount, "format_count", Target::Enum, [], No, Inherited;

    // panicking
    PanicLocation, "panic_location", Target::Struct, [Generics(1) /* lifetime */], No, Inherited;
    PanicInfo, "panic_info", Target::Struct, [Generics(1) /* lifetime */], No, Inherited;
    Panic, "panic", Target::Fn, [Arguments(1), Constness], No, Inherited;
    PanicImpl, "panic_impl", Target::Fn, [Arguments(1)], No, Inherited;
    PanicFmt, "panic_fmt", Target::Fn, [Arguments(1), Constness], No, Inherited;
    PanicNounwind, "panic_nounwind", Target::Fn, [Arguments(1), Constness], No, Inherited;
    PanicBoundsCheck, "panic_bounds_check", Target::Fn, [Arguments(2)], No, Inherited;
    PanicMisalignedPointerDereference, "panic_misaligned_pointer_dereference", Target::Fn, [Arguments(2)], No, Inherited;
    PanicNullPointerDereference, "panic_null_pointer_dereference", Target::Fn, [Arguments(0)], No, Inherited;
    PanicCannotUnwind, "panic_cannot_unwind", Target::Fn, [Arguments(0)], No, Inherited;
    PanicInCleanup, "panic_in_cleanup", Target::Fn, [Arguments(0)], No, Inherited;
    ConstPanicFmt, "const_panic_fmt", Target::Fn, [Arguments(1), Constness], No, Inherited;

    // ffis
    CStr, "cstr", Target::Struct, [Transparent], Named("cstr_type"), Named("CStr");
    CVoid, "cvoid", Target::Enum, [CLayout], No, Named("c_void");

    // TEST ITEMS
    #[cfg(feature = "test_lang_item")]
    TestAuto, "foo_auto", Target::Trait, [Auto], No, No;
    #[cfg(feature = "test_lang_item")]
    TestConstness, "foo_const", Target::Trait, [Constness], No, No;
    #[cfg(feature = "test_lang_item")]
    TestUnsafety, "foo_unsafe", Target::Trait, [Unsafety], No, No;
    #[cfg(feature = "test_lang_item")]
    TestFnUnsafety, "foo_fn_unsafe", Target::Fn, [Unsafety], No, No;
    #[cfg(feature = "test_lang_item")]
    TestFn, "foo_fn", Target::Fn, [Arguments(1)], No, No;
}
