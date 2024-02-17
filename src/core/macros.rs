#[macro_export]
macro_rules! matches {
    ($cond:expr, $pat:pat $(if $guard:expr)? $(,)?) => {
        match $cond {
            $pat $(if $guard)? => true,
            _ => false
        }
    };
}
