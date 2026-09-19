//! Implementation crate for the `#[slice_struct]` procedural macro.
//!
//! **Do not depend on this crate directly.** Use the `slice_struct` crate,
//! which re-exports the macro.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod codegen;
mod parse;

/// Transform a struct so that fields marked `#[slice]` become inline,
/// variable-length slices packed into a single heap allocation.
#[proc_macro_attribute]
pub fn slice_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let zerocopy = attr_str.contains("zerocopy");
    let unpin = attr_str.contains("unpin") || zerocopy;
    let arena = attr_str.contains("arena");
    let input = parse_macro_input!(item as syn::ItemStruct);
    let mut parsed = parse::parse_slice_struct(input);
    parsed.unpin = unpin;
    parsed.zerocopy = zerocopy;
    parsed.is_arena = arena;
    let expanded = codegen::generate(&parsed);
    expanded.into()
}
