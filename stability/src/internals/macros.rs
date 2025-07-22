macro_rules! find_attr {
    (type bool) => {
        $crate::internals::BoolAttr
    };
    (value bool, $name:expr, $field:expr, $ctx:expr, $input:expr) => {
        if $input.peek(syn::Token![=]) {
            match get_value::<syn::LitBool>($input) {
                Ok(syn::LitBool { value: true, .. }) => $field.set_true($name),
                Ok(syn::LitBool { value: false, .. }) => $field.set_false($name),
                Err(err) => $ctx.push_error(err),
            }
        } else {
            $field.set_true($name);
        }
    };
    (type $ty:ty) => {
        $crate::internals::Attr::<$ty>
    };
    (value String, $name:expr, $field:expr, $ctx:expr, $input:expr) => {
        match get_value::<syn::LitStr>($input) {
            Ok(value) => $field.set($name, value.value()),
            Err(err) => $ctx.push_error(err),
        }
    };
}

macro_rules! parsable_attribute {
    ($vis:vis struct $name:ident {
        $($arg_vis:vis $arg:ident : $ty:tt = $symb:expr),* $(,)?
    }) => {
        $vis struct $name {
            $($arg_vis $arg: $ty,)*

            pub ctx: Context,
        }

        impl $name {
            pub fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                use syn::spanned::Spanned;
                use $crate::internals::macros::find_attr;

                let ctx = $crate::internals::Context::default();
                $(let mut $arg = <find_attr!(type $ty)>::none(&ctx, $symb);)*

                let mut first = true;
                while !input.is_empty() {
                    if !first {
                        input.parse::<syn::Token![,]>()?;
                        if input.is_empty() {
                            break;
                        }
                    }
                    first = false;

                    let name: syn::Ident = input.parse()?;
                    $(if $symb == name {
                        find_attr![value $ty, name, $arg, ctx, &input];
                        continue;
                    })*

                    if input.peek(syn::Token![=]) {
                        match get_value::<syn::Expr>(&input) {
                            Ok(expr) => {
                                let span = expr.span().join(name.span()).unwrap_or(proc_macro2::Span::call_site());
                                ctx.push_error(syn::Error::new(span, "unrecognized argument"));
                            },
                            Err(err) => ctx.push_error(err),
                        }
                    } else {
                        ctx.push_error_spanned_by(name, "unrecognized argument");
                    }
                }

                Ok(Self {
                    $($arg: $arg.get(),)*

                    ctx,
                })
            }
        }
    };
}

pub(crate) use {find_attr, parsable_attribute};
