use crate::parse::{SliceField, SliceStructInput};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let inner_struct_name = format_ident!("{}__Inner", struct_name);

    let mut calc_args = quote! {};
    let mut offset_idents = Vec::new();
    let mut layout_ret_types = Vec::new();

    for field in &input.slice_fields {
        let id = field.ident();
        let len_ident = format_ident!("{}_len", id);
        offset_idents.push(format_ident!("{}_offset", id));
        layout_ret_types.push(quote!(usize));

        match field {
            SliceField::Flat { .. } => {
                calc_args.extend(quote! { #len_ident: usize, });
            }
            SliceField::Arena { inner_ty, .. } => {
                let arena_ident = format_ident!("{}_arena", id);
                calc_args.extend(quote! {
                    #len_ident: usize,
                    #arena_ident: &<#inner_ty as ::slice_struct::ArenaElement>::Arena,
                });
            }
        }
    }

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
    for field in &input.slice_fields {
        let ty = field.ty();
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
    for field in &input.slice_fields {
        let ident = field.ident();
        let len_ident = format_ident!("{}_len", ident);
        let offset_ident = format_ident!("{}_offset", ident);
        match field {
            SliceField::Flat { ty, .. } => {
                layout_stmts.extend(quote! {
                    let (layout, #offset_ident) =
                        layout.extend(::std::alloc::Layout::array::<<#ty as ::slice_struct::__private::InlineSlice>::Element>(#len_ident).unwrap()).unwrap();
                });
            }
            SliceField::Arena { inner_ty, .. } => {
                let arena_ident = format_ident!("{}_arena", ident);
                layout_stmts.extend(quote! {
                    let __arena_byte_size = #len_ident * #arena_ident.instance_size();
                    let (layout, #offset_ident) =
                        layout.extend(::std::alloc::Layout::from_size_align(
                            __arena_byte_size,
                            <#inner_ty as ::slice_struct::ArenaElement>::Arena::instance_align()
                        ).unwrap()).unwrap();
                });
            }
        }
    }
    layout_stmts.extend(quote! {
        (layout.pad_to_align(), #(#offset_idents),*)
    });

    let layout_helper_ident = format_ident!("{}_LayoutHelper", struct_name);

    quote! {
        #[allow(non_camel_case_types)]
        struct #layout_helper_ident #impl_generics (::core::marker::PhantomData<#struct_name #ty_generics>) #where_clause;

        impl #impl_generics #layout_helper_ident #ty_generics #where_clause {
            fn calculate_layout(#calc_args)
                -> (::std::alloc::Layout, #(#layout_ret_types),*)
            {
                #layout_stmts
            }
        }
    }
}
