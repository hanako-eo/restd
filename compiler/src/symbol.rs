pub const REPR: Symbol = Symbol("repr");
pub const C: Symbol = Symbol("C");
pub const TRANSPARENT: Symbol = Symbol("transparent");

#[derive(Debug)]
pub struct Symbol(pub &'static str);

impl PartialEq<syn::Path> for Symbol {
    fn eq(&self, other: &syn::Path) -> bool {
        if self.0.split("::").count() != other.segments.len() {
            return false;
        }

        self.0
            .split("::")
            .zip(other.segments.iter())
            .all(|(self_segment, other_segment)| other_segment.ident == self_segment)
    }
}

impl PartialEq<syn::Ident> for Symbol {
    fn eq(&self, other: &syn::Ident) -> bool {
        other == self.0
    }
}
