#[rustc_paren_sugar]
#[lang = "fn_mut"]
pub trait FnMut<Args>: Fn<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[rustc_paren_sugar]
#[lang = "fn"]
pub trait Fn<Args>: FnOnce<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

#[rustc_paren_sugar]
#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}
