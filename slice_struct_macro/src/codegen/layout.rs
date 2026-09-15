use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let sized_prefix_ident = format_ident!("{}_SizedPrefix", struct_name);

    let len_idents_only: Vec<_> = input.slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_len", id))
        .collect();
    let offset_idents: Vec<_> = input.slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_offset", id))
        .collect();
    let layout_ret_types: Vec<_> = input.slice_fields.iter().map(|_| quote!(usize)).collect();

    let mut layout_stmts = quote! {
        let layout = ::std::alloc::Layout::new::<#sized_prefix_ident #ty_generics>();
    };
    for (ident, ty, _) in &input.slice_fields {
        let len_ident = format_ident!("{}_len", ident);
        let offset_ident = format_ident!("{}_offset", ident);
        layout_stmts.extend(quote! {
            let (layout, #offset_ident) =
                layout.extend(::std::alloc::Layout::array::<#ty>(#len_ident).unwrap()).unwrap();
        });
    }
    layout_stmts.extend(quote! {
        (layout.pad_to_align(), #(#offset_idents),*)
    });

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc(hidden)]
            #[inline]
            pub(crate) fn __layout(#(#len_idents_only: usize),*)
                -> (::std::alloc::Layout, #(#layout_ret_types),*)
            {
                #layout_stmts
            }
        }
    }
}
