pub mod structs;
pub mod view;
pub mod layout;
pub mod init;

use proc_macro2::TokenStream;
use quote::quote;
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let (private_structs, public_structs) = structs::generate(input);
    let view = view::generate(input);
    let layout = layout::generate(input);
    let init = init::generate(input);

    let struct_name = &input.struct_name;
    let inner_struct_name = quote::format_ident!("{}__Inner", struct_name);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    
    let zerocopy_methods = if input.zerocopy {
        quote! {
            #[cfg(feature = "zero_copy")]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #[doc = "Safely cast a byte slice into a reference of this struct."]
                pub fn ref_from_bytes(bytes: &[u8]) -> ::core::option::Option<&Self> {
                    let inner_ref = ::slice_struct::__private::zerocopy::Ref::<&[u8], #inner_struct_name #ty_generics>::from_bytes(bytes).ok()?;
                    let inner: &#inner_struct_name #ty_generics = ::slice_struct::__private::zerocopy::Ref::into_ref(inner_ref);
                    ::core::option::Option::Some(unsafe { &*(inner as *const _ as *const Self) })
                }
                #[doc = "Safely cast a mutable byte slice into a mutable reference of this struct."]
                pub fn mut_from_bytes(bytes: &mut [u8]) -> ::core::option::Option<&mut Self> {
                    let inner_ref = ::slice_struct::__private::zerocopy::Ref::<&[u8], #inner_struct_name #ty_generics>::from_bytes(bytes).ok()?;
                    let inner: &#inner_struct_name #ty_generics = ::slice_struct::__private::zerocopy::Ref::into_ref(inner_ref);
                    let fat_ptr_const = inner as *const _ as *const Self;
                    let len = unsafe { &*(fat_ptr_const as *const [()]) }.len();
                    let fat_ptr_mut = ::core::ptr::slice_from_raw_parts_mut(bytes.as_mut_ptr() as *mut (), len) as *mut Self;
                    ::core::option::Option::Some(unsafe { &mut *fat_ptr_mut })
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #public_structs
        #zerocopy_methods
        #view
        const _: () = {
            #private_structs
            #layout
            #init
        };
    }
}
