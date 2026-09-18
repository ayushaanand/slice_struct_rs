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

    let mode = if input.unpin {
        quote! { ::slice_struct::RelativeMode }
    } else {
        quote! { ::slice_struct::AbsoluteMode }
    };

    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);

        prefix_fields.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::State,
            #align_ident: [<#ty as ::slice_struct::InlineSlice>::Element; 0],
            #internal_ident: ::slice_struct::SliceHandle<<#ty as ::slice_struct::InlineSlice>::Element, #mode>,
        });
        actual_fields.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::State,
            #align_ident: [<#ty as ::slice_struct::InlineSlice>::Element; 0],
            #internal_ident: ::slice_struct::SliceHandle<<#ty as ::slice_struct::InlineSlice>::Element, #mode>,
        });
    }
    
    prefix_fields.extend(quote! {
        __pin: <#mode as ::slice_struct::AddressingMode>::Marker,
    });

    let data_field = quote! {
        __pin: <#mode as ::slice_struct::AddressingMode>::Marker,
        #[doc(hidden)]
        pub __data_tail: [::core::mem::MaybeUninit<u8>]
    };

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut drop_slices = quote! {};
    for (ident, ty, _) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        drop_slices.extend(quote! {
            unsafe {
                let base_ptr = self as *mut _ as *mut u8;
                let data = self.#internal_ident.as_non_null(base_ptr);
                ::core::ptr::drop_in_place(data.as_ptr());
            }
        });
    }

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
        
        impl #impl_generics ::core::ops::Drop for #struct_name #ty_generics #where_clause {
            fn drop(&mut self) {
                #drop_slices
            }
        }
    }
}
