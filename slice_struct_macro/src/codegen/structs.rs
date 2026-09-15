use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let vis = &input.vis;

    
    let sized_prefix_ident = format_ident!("{}_SizedPrefix", struct_name);

    let mut prefix_fields = quote! {};
    let mut actual_fields = quote! {};

    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        let ty = &field.ty;
        prefix_fields.extend(quote! { #internal_ident: #ty, });
        actual_fields.extend(quote! { #internal_ident: #ty, });
    }

    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);

        prefix_fields.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::State,
            #align_ident: [<#ty as ::slice_struct::InlineSlice>::Element; 0],
            #internal_ident: ::slice_struct::SliceHandle<<#ty as ::slice_struct::InlineSlice>::Element>,
        });
        actual_fields.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::State,
            #align_ident: [<#ty as ::slice_struct::InlineSlice>::Element; 0],
            #internal_ident: ::slice_struct::SliceHandle<<#ty as ::slice_struct::InlineSlice>::Element>,
        });
    }
    
    prefix_fields.extend(quote! {
        __pin: ::core::marker::PhantomPinned,
    });

    let data_field = quote! {
        __pin: ::core::marker::PhantomPinned,
        #[doc(hidden)]
        pub __data_tail: [::core::mem::MaybeUninit<u8>]
    };

    let generics = &input.generics;

    quote! {
        #[repr(C)]
        #[doc(hidden)]
        #vis struct #sized_prefix_ident #generics {
            #prefix_fields
        }

        #[repr(C)]
        #vis struct #struct_name #generics {
            #actual_fields
            #data_field
        }
    }
}
