pub mod option;
pub mod result;

#[macro_export]
macro_rules! matches {
    ($ty:expr, $case:pat) => {
        match $ty {
            $case => true,
            _ => false
        }
    };
}
