pub mod structs;
pub mod view;
pub mod layout;
pub mod init;

use proc_macro2::TokenStream;
use quote::quote;
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let structs = structs::generate(input);
    let view = view::generate(input);
    let layout = layout::generate(input);
    let init = init::generate(input);

    quote! {
        #structs
        #view
        #layout
        #init
    }
}
