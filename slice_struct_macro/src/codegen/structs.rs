use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> (TokenStream, TokenStream) {
    let struct_name = &input.struct_name;
    let vis = &input.vis;

    let mut actual_fields = quote! {};

    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        let ty = &field.ty;
        actual_fields.extend(quote! { #[doc(hidden)] #internal_ident: #ty, });
    }

    let mode = if input.unpin {
        quote! { ::slice_struct::__private::RelativeMode }
    } else {
        quote! { ::slice_struct::__private::AbsoluteMode }
    };

    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);

        actual_fields.extend(quote! {
            #[doc(hidden)] #state_ident: <#ty as ::slice_struct::__private::InlineSlice>::State,
            #[doc(hidden)] #align_ident: [<#ty as ::slice_struct::__private::InlineSlice>::Element; 0],
            #[doc(hidden)] #internal_ident: ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, #mode>,
        });
    }

    let data_tail_type = if input.zerocopy {
        quote! { ::core::mem::MaybeUninit<u8> }
    } else {
        quote! { ::slice_struct::__private::__SyncUnsafeCell<::core::mem::MaybeUninit<u8>> }
    };

    let data_field = quote! {
        #[doc(hidden)] __pin: <#mode as ::slice_struct::__private::AddressingMode>::Marker,
        #[doc(hidden)] __tail_start: [u8; 0],
        #[doc(hidden)] __data_tail: [#data_tail_type]
    };

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut drop_slices = quote! {};
    for (ident, _, _) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        drop_slices.extend(quote! {
            unsafe {
                let base_ptr = self as *mut _ as *mut u8;
                let data = self.0.#internal_ident.as_non_null(base_ptr);
                ::core::ptr::drop_in_place(data.as_ptr());
            }
        });
    }

    let private_structs = quote! {};

    let inner_struct_name = format_ident!("{}__Inner", struct_name);

    let zerocopy_derives_inner = if input.zerocopy {
        quote! { #[cfg_attr(feature = "zero_copy", derive(::slice_struct::__private::zerocopy::FromBytes, ::slice_struct::__private::zerocopy::KnownLayout, ::slice_struct::__private::zerocopy::Immutable))] }
    } else {
        quote! {}
    };

    let zerocopy_derives_outer = if input.zerocopy {
        quote! { #[derive(::slice_struct::__private::zerocopy::FromBytes, ::slice_struct::__private::zerocopy::Immutable)] }
    } else {
        quote! {}
    };

    let public_structs = quote! {
        #zerocopy_derives_inner
        #[repr(C)]
        #[doc(hidden)]
        #vis struct #inner_struct_name #generics {
            #actual_fields
            #data_field
        }

        #zerocopy_derives_outer
        #[repr(transparent)]
        #vis struct #struct_name #generics(#inner_struct_name #ty_generics);
        
        impl #impl_generics ::core::ops::Drop for #struct_name #ty_generics #where_clause {
            fn drop(&mut self) {
                #drop_slices
            }
        }
    };

    (private_structs, public_structs)
}
