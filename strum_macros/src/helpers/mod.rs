pub use self::case_style::{snakify, CaseStyleHelpers};
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

pub mod case_style;
mod metadata;
pub mod type_props;
pub mod variant_props;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn strum_discriminants_passthrough_error(span: &impl Spanned) -> syn::Error {
    syn::Error::new(
        span.span(),
        "expected a pass-through attribute, e.g. #[strum_discriminants(serde(rename = \"var0\"))]",
    )
}

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}
