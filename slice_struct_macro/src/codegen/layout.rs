use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let inner_struct_name = format_ident!("{}__Inner", struct_name);

    let len_idents_only: Vec<_> = input.slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_len", id))
        .collect();
    let offset_idents: Vec<_> = input.slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_offset", id))
        .collect();
    let layout_ret_types: Vec<_> = input.slice_fields.iter().map(|_| quote!(usize)).collect();

    let mut align_types = quote! {};
    for field in &input.sized_fields {
        let ty = &field.ty;
        align_types.extend(quote! { #ty, });
    }
    let mode = if input.unpin {
        quote! { ::slice_struct::__private::RelativeMode }
    } else {
        quote! { ::slice_struct::__private::AbsoluteMode }
    };
    for (_, ty, _) in &input.slice_fields {
        align_types.extend(quote! {
            <#ty as ::slice_struct::__private::InlineSlice>::State,
            <#ty as ::slice_struct::__private::InlineSlice>::Element,
            ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, #mode>,
        });
    }
    align_types.extend(quote! { <#mode as ::slice_struct::__private::AddressingMode>::Marker, });

    let mut layout_stmts = quote! {
        let base_offset = ::core::mem::offset_of!(#inner_struct_name #ty_generics, __tail_start);
        let base_align = ::core::mem::align_of::<( #align_types )>();
        let layout = ::std::alloc::Layout::from_size_align(base_offset, base_align).unwrap();
    };
    for (ident, ty, _) in &input.slice_fields {
        let len_ident = format_ident!("{}_len", ident);
        let offset_ident = format_ident!("{}_offset", ident);
        layout_stmts.extend(quote! {
            let (layout, #offset_ident) =
                layout.extend(::std::alloc::Layout::array::<<#ty as ::slice_struct::__private::InlineSlice>::Element>(#len_ident).unwrap()).unwrap();
        });
    }
    layout_stmts.extend(quote! {
        (layout.pad_to_align(), #(#offset_idents),*)
    });

    let layout_helper_ident = format_ident!("{}_LayoutHelper", struct_name);
    
    quote! {
        #[allow(non_camel_case_types)]
        struct #layout_helper_ident #impl_generics (::core::marker::PhantomData<#struct_name #ty_generics>) #where_clause;
        
        impl #impl_generics #layout_helper_ident #ty_generics #where_clause {
            fn calculate_layout(#(#len_idents_only: usize),*)
                -> (::std::alloc::Layout, #(#layout_ret_types),*)
            {
                #layout_stmts
            }
        }
    }
}
