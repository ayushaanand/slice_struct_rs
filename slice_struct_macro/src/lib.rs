//! Implementation crate for the `#[slice_struct]` procedural macro.
//!
//! **Do not depend on this crate directly.** Use the `slice_struct` crate,
//! which re-exports the macro.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod parse;
mod codegen;

/// Transform a struct so that fields marked `#[slice]` become inline,
/// variable-length slices packed into a single heap allocation.
#[proc_macro_attribute]
pub fn slice_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let parsed = parse::parse_slice_struct(input);
    let expanded = codegen::generate(&parsed);
    expanded.into()
}
