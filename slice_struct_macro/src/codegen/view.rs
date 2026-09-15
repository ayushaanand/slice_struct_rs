use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut view_generics = input.generics.clone();
    view_generics.params.insert(0, ::syn::parse_quote!('__a));
    for (_, ty, _) in &input.slice_fields {
        view_generics.make_where_clause().predicates.push(::syn::parse_quote!(#ty: '__a));
    }
    let (_view_impl_generics, view_ty_generics, view_where_clause) = view_generics.split_for_impl();
    
    let view_ident = format_ident!("{}View", struct_name);
    let view_mut_ident = format_ident!("{}ViewMut", struct_name);

    let mut view_fields = quote! {};
    let mut view_init = quote! {};
    
    let mut view_mut_fields = quote! {};
    let mut view_mut_init = quote! {};

    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        let ty = &field.ty;
        let fvis = &field.vis;
        
        view_fields.extend(quote! { #fvis #ident: &'__a #ty, });
        view_init.extend(quote! { #ident: &this.#internal_ident, });
        
        view_mut_fields.extend(quote! { #fvis #ident: &'__a mut #ty, });
        view_mut_init.extend(quote! { #ident: &mut this.#internal_ident, });
    }

    for (ident, ty, fvis) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);
        
        view_fields.extend(quote! { #fvis #ident: <#ty as ::slice_struct::InlineSlice>::View<'__a>, });
        view_init.extend(quote! { 
            #ident: <#ty as ::slice_struct::InlineSlice>::project(
                &this.#state_ident,
                this.#internal_ident.as_non_null()
            ),
        });
        
        view_mut_fields.extend(quote! { #fvis #ident: <#ty as ::slice_struct::InlineSlice>::ViewMut<'__a>, });
        view_mut_init.extend(quote! {
            #ident: <#ty as ::slice_struct::InlineSlice>::project_mut(
                &this.#state_ident,
                this.#internal_ident.as_non_null()
            ),
        });
    }

    quote! {
        #[doc = "An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields."]
        #vis struct #view_ident #view_generics #view_where_clause {
            #view_fields
        }

        #[doc = "A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.\n\nThis struct enables safe disjoint borrowing of multiple slices simultaneously."]
        #vis struct #view_mut_ident #view_generics #view_where_clause {
            #view_mut_fields
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = "Returns a struct containing immutable references to all fields."]
            #vis fn view<'__a>(&'__a self) -> #view_ident #view_ty_generics {
                let this = self;
                #view_ident {
                    #view_init
                }
            }

            #[doc = "Returns a struct containing mutable references to all fields, allowing safe disjoint borrowing of the slice fields."]
            #vis fn view_mut<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> #view_mut_ident #view_ty_generics {
                unsafe {
                    let this = self.get_unchecked_mut();
                    #view_mut_ident {
                        #view_mut_init
                    }
                }
            }
        }
    }
}
