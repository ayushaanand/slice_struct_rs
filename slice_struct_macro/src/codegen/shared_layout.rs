use crate::parse::SliceStructInput;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(input: &SliceStructInput) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if !input.shared_layout {
        return (quote! {}, quote! {});
    }

    let struct_name = &input.struct_name;
    let vis = &input.vis;
    let table_name = format_ident!("{}LayoutData", struct_name);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut table_fields = quote! {
        __layout: ::std::alloc::Layout,
    };
    let mut offset_vars = Vec::new();
    let mut len_vars = Vec::new();
    
    let mut make_args = quote! {};
    let mut layout_args = quote! {};
    let mut struct_inits = quote! {};

    for field in &input.slice_fields {
        let ident = field.ident();
        let ty = field.ty();
        let len_ident = format_ident!("{}_len", ident);
        let offset_ident = format_ident!("{}_offset", ident);
        let state_ident = format_ident!("{}_state", ident);
        
        table_fields.extend(quote! {
            pub #len_ident: usize,
            pub #offset_ident: usize,
            pub #state_ident: <#ty as ::slice_struct::__private::InlineSlice>::State,
        });

        match field {
            crate::parse::SliceField::Flat { .. } => {
                make_args.extend(quote! { #len_ident: usize, });
                struct_inits.extend(quote! {
                    #len_ident,
                    #offset_ident,
                    #state_ident: <#ty as ::slice_struct::__private::InlineSlice>::init_state(),
                });
            }
            crate::parse::SliceField::Arena { inner_ty, .. } => {
                let arena_ident = format_ident!("{}_arena", ident);
                make_args.extend(quote! { #len_ident: usize, #arena_ident: <#inner_ty as ::slice_struct::ArenaElement>::Arena, });
                struct_inits.extend(quote! {
                    #len_ident,
                    #offset_ident,
                    #state_ident: #arena_ident,
                });
            }
        }
        
        offset_vars.push(offset_ident);
        len_vars.push(len_ident);
    }
    
    // We reuse the SliceInit trait's calculate_layout statically
    let turbofish = if input.generics.params.is_empty() {
        quote! {}
    } else {
        let tys: Vec<_> = input.generics.params.iter().map(|p| match p {
            syn::GenericParam::Type(t) => { let id = &t.ident; quote!{ #id } },
            syn::GenericParam::Lifetime(l) => { let l = &l.lifetime; quote!{ #l } },
            syn::GenericParam::Const(c) => { let id = &c.ident; quote!{ #id } },
        }).collect();
        quote! { ::<#(#tys),*> }
    };
    let layout_helper = format_ident!("{}_LayoutHelper", struct_name);

    for field in &input.slice_fields {
        let ident = field.ident();
        let len_ident = format_ident!("{}_len", ident);
        match field {
            crate::parse::SliceField::Flat { .. } => {
                layout_args.extend(quote! { #len_ident, });
            }
            crate::parse::SliceField::Arena { .. } => {
                let arena_ident = format_ident!("{}_arena", ident);
                layout_args.extend(quote! { #len_ident, &#arena_ident, });
            }
        }
    }

    (
        quote! {
            #[doc(hidden)]
            #vis struct #table_name #impl_generics #where_clause {
                #table_fields
            }
        },
        quote! {
            impl #impl_generics ::slice_struct::SharedLayoutData for #table_name #ty_generics #where_clause {
                #[inline]
                fn total_layout(&self) -> ::std::alloc::Layout {
                    self.__layout
                }
            }
            
            impl #impl_generics ::slice_struct::SharedLayout for #struct_name #ty_generics #where_clause {
                type LayoutData = #table_name #ty_generics;
            }

            impl #impl_generics #struct_name #ty_generics #where_clause {
                #[doc = "Create a shared layout table for multiple struct instances."]
                #vis fn make_table(#make_args) -> ::std::sync::Arc<::slice_struct::LayoutTable<Self>> {
                    let (layout, #(#offset_vars),*) = #layout_helper #turbofish::calculate_layout(#layout_args);
                    ::std::sync::Arc::new(::slice_struct::LayoutTable {
                        data: #table_name {
                            __layout: layout,
                            #struct_inits
                        }
                    })
                }
            }
        }
    )
}
