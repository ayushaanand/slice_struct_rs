use crate::parse::{SliceField, SliceStructInput};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let vis = &input.vis;
    let arena_name = format_ident!("{}Arena", struct_name);
    let inner_struct_name = format_ident!("{}__Inner", struct_name);
    let layout_helper = format_ident!("{}_LayoutHelper", struct_name);
    let view_name = format_ident!("{}View", struct_name);
    let view_mut_name = format_ident!("{}ViewMut", struct_name);

    let mut arena_fields = quote! {};
    let mut arena_fn_args = quote! {};
    let mut arena_fn_init = quote! {};
    let mut calc_args = quote! {};

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut init_data_tuple = quote! {};
    let mut write_def_stmts = quote! {};

    let mut offset_idents = Vec::new();
    let mut tuple_idx = 0usize;

    for field in &input.sized_fields {
        let id = field.ident.as_ref().unwrap();
        let internal_id = format_ident!("__{}", id);
        let ty = &field.ty;
        let idx = syn::Index::from(tuple_idx);
        init_data_tuple.extend(quote! { #ty, });
        write_def_stmts.extend(quote! {
            ::core::ptr::write(
                ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *mut #ty,
                ::core::clone::Clone::clone(&data.#idx)
            );
        });
        tuple_idx += 1;
    }

    for (i, field) in input.slice_fields.iter().enumerate() {
        let id = field.ident();
        let len_ident = format_ident!("{}_len", id);
        let state_ident = format_ident!("__{}_state", id);
        let align_ident = format_ident!("__align_{}", i);
        let internal_id = format_ident!("__{}", id);
        let ty = field.ty();
        let offset_ident = format_ident!("{}_offset", id);
        offset_idents.push(offset_ident.clone());
        let idx = syn::Index::from(tuple_idx);

        match field {
            SliceField::Flat { ty, .. } => {
                arena_fields.extend(quote! { pub #len_ident: usize, });
                arena_fn_args.extend(quote! { #len_ident: usize, });
                arena_fn_init.extend(quote! { #len_ident, });
                calc_args.extend(quote! { self.#len_ident, });
                init_data_tuple
                    .extend(quote! { <#ty as ::slice_struct::__private::InlineSlice>::Element, });

                write_def_stmts.extend(quote! {
                    let field_len = self.#len_ident;
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #align_ident)) as *mut [<#ty as ::slice_struct::__private::InlineSlice>::Element; 0], []);
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #state_ident)) as *mut <#ty as ::slice_struct::__private::InlineSlice>::State, <#ty as ::slice_struct::__private::InlineSlice>::init_state());
                    
                    let handle = ::slice_struct::__private::SliceHandle::<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>::__new_unchecked(
                        <::slice_struct::__private::RelativeMode as ::slice_struct::__private::AddressingMode>::store::<<#ty as ::slice_struct::__private::InlineSlice>::Element>(ptr, #offset_ident),
                        field_len
                    );
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *mut ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>, handle);
                    
                    if field_len > 0 {
                        let field_ptr = ptr.add(#offset_ident).cast::<<#ty as ::slice_struct::__private::InlineSlice>::Element>();
                        let def_val = &data.#idx;
                        for j in 0..field_len - 1 {
                            ::core::ptr::write(field_ptr.add(j), ::core::clone::Clone::clone(def_val));
                        }
                        ::core::ptr::write(field_ptr.add(field_len - 1), ::core::clone::Clone::clone(def_val));
                    }
                });
            }
            SliceField::Arena { inner_ty, .. } => {
                let arena_ident = format_ident!("{}_arena", id);
                arena_fields.extend(quote! {
                    pub #arena_ident: <#inner_ty as ::slice_struct::ArenaElement>::Arena,
                    pub #len_ident: usize,
                });
                arena_fn_args.extend(quote! {
                    #len_ident: usize,
                    #arena_ident: <#inner_ty as ::slice_struct::ArenaElement>::Arena,
                });
                arena_fn_init.extend(quote! { #len_ident, #arena_ident, });
                calc_args.extend(quote! { self.#len_ident, &self.#arena_ident, });
                init_data_tuple.extend(quote! { <<#inner_ty as ::slice_struct::ArenaElement>::Arena as ::slice_struct::ArenaDescriptor>::InitData, });

                write_def_stmts.extend(quote! {
                    let field_len = self.#len_ident;
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #align_ident)) as *mut [<#ty as ::slice_struct::__private::InlineSlice>::Element; 0], []);
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #state_ident)) as *mut <#ty as ::slice_struct::__private::InlineSlice>::State, ::core::clone::Clone::clone(&self.#arena_ident));
                    
                    let handle = ::slice_struct::__private::SliceHandle::<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>::__new_unchecked(
                        <::slice_struct::__private::RelativeMode as ::slice_struct::__private::AddressingMode>::store::<<#ty as ::slice_struct::__private::InlineSlice>::Element>(ptr, #offset_ident),
                        field_len
                    );
                    ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *mut ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>, handle);
                    
                    if field_len > 0 {
                        let field_ptr = ptr.add(#offset_ident);
                        let def_val = &data.#idx;
                        let stride = self.#arena_ident.instance_size();
                        for j in 0..field_len {
                            self.#arena_ident.write_instance_def(field_ptr.add(j * stride), def_val);
                        }
                    }
                });
            }
        }
        tuple_idx += 1;
    }

    // add tail writes
    write_def_stmts.extend(quote! {
        ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, __pin)) as *mut <::slice_struct::__private::RelativeMode as ::slice_struct::__private::AddressingMode>::Marker, <::slice_struct::__private::RelativeMode as ::slice_struct::__private::AddressingMode>::MARKER_INIT);
        ::core::ptr::write(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, __tail_start)) as *mut [u8; 0], []);
    });

    let mut drop_def_stmts = quote! {};
    for field in &input.sized_fields {
        let ty = &field.ty;
        let id = field.ident.as_ref().unwrap();
        let internal_id = format_ident!("__{}", id);
        drop_def_stmts.extend(quote! {
            ::core::ptr::drop_in_place(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *mut #ty);
        });
    }
    for field in &input.slice_fields {
        let id = field.ident();
        let ty = field.ty();
        let internal_id = format_ident!("__{}", id);

        let state_val = match field {
            crate::parse::SliceField::Flat { .. } => {
                let state_id = format_ident!("__{}_state", id);
                quote! { &*(ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #state_id)) as *const _) }
            },
            crate::parse::SliceField::Arena { .. } => {
                let arena_id = format_ident!("{}_arena", id);
                quote! { &self.#arena_id }
            }
        };

        drop_def_stmts.extend(quote! {
            let handle_ptr = ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *const ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>;
            let data = (&*handle_ptr).as_non_null(ptr);
            <#ty as ::slice_struct::__private::InlineSlice>::drop_slice(#state_val, data);
        });
    }

    let mut align_queries = quote! {};
    for field in &input.sized_fields {
        let ty = &field.ty;
        align_queries.extend(quote! {
            let a = ::core::mem::align_of::<#ty>();
            if a > max_align { max_align = a; }
        });
    }
    for field in &input.slice_fields {
        let ty = field.ty();
        align_queries.extend(quote! {
            let a = ::core::mem::align_of::<<#ty as ::slice_struct::__private::InlineSlice>::State>();
            if a > max_align { max_align = a; }
            let a = ::core::mem::align_of::<<#ty as ::slice_struct::__private::InlineSlice>::Element>();
            if a > max_align { max_align = a; }
            let a = ::core::mem::align_of::<::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>>();
            if a > max_align { max_align = a; }
        });
    }
    align_queries.extend(quote! {
        let a = ::core::mem::align_of::<<::slice_struct::__private::RelativeMode as ::slice_struct::__private::AddressingMode>::Marker>();
        if a > max_align { max_align = a; }
    });

    let init_data_type = quote! { ( #init_data_tuple ) };

    let mut project_fields = quote! {};
    let mut project_mut_fields = quote! {};
    for field in &input.sized_fields {
        let id = field.ident.as_ref().unwrap();
        let internal_id = format_ident!("__{}", id);
        project_fields.extend(quote! {
            #id: &*(base_ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *const _),
        });
        project_mut_fields.extend(quote! {
            #id: &mut *(base_mut.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *mut _),
        });
    }
    for field in &input.slice_fields {
        let id = field.ident();
        let internal_id = format_ident!("__{}", id);
        let ty = field.ty();
        let state_val = match field {
            crate::parse::SliceField::Flat { .. } => {
                let state_id = format_ident!("__{}_state", id);
                quote! { *(base_ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #state_id)) as *const _) }
            },
            crate::parse::SliceField::Arena { .. } => {
                let arena_id = format_ident!("{}_arena", id);
                quote! { arena.#arena_id }
            }
        };
        project_fields.extend(quote! {
            #id: <#ty as ::slice_struct::__private::InlineSlice>::project(
                &#state_val,
                (&*(base_ptr.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *const ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>)).as_non_null(base_ptr)
            ),
        });
        project_mut_fields.extend(quote! {
            #id: <#ty as ::slice_struct::__private::InlineSlice>::project_mut(
                &#state_val,
                (&*(base_mut.add(::core::mem::offset_of!(#inner_struct_name #ty_generics, #internal_id)) as *const ::slice_struct::__private::SliceHandle<<#ty as ::slice_struct::__private::InlineSlice>::Element, ::slice_struct::__private::RelativeMode>)).as_non_null(base_mut)
            ),
        });
    }

    let zerocopy_derive = if input.zerocopy {
        quote! {
            #[cfg_attr(feature = "zero_copy", derive(
                ::slice_struct::__private::zerocopy::FromBytes,
                ::slice_struct::__private::zerocopy::IntoBytes,
                ::slice_struct::__private::zerocopy::Immutable,
                ::slice_struct::__private::zerocopy::KnownLayout
            ))]
        }
    } else {
        quote! {}
    };

    quote! {
        #zerocopy_derive
        pub struct #arena_name {
            #arena_fields
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = "Initialize the arena descriptor for this struct."]
            #vis fn init_arena(#arena_fn_args) -> #arena_name {
                #arena_name {
                    #arena_fn_init
                }
            }
        }

        impl ::core::clone::Clone for #arena_name {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for #arena_name {}

        impl ::slice_struct::ArenaDescriptor for #arena_name {
            type View<'a> = #view_name<'a>;
            type ViewMut<'a> = #view_mut_name<'a>;
            type InitData = #init_data_type;

            fn instance_size(&self) -> usize {
                #layout_helper::calculate_layout(#calc_args).0.size()
            }

            fn instance_align() -> usize {
                let mut max_align = 1;
                #align_queries
                max_align
            }

            unsafe fn write_instance_def(&self, ptr: *mut u8, data: &Self::InitData) {
                let (_, #(#offset_idents),*) = #layout_helper::calculate_layout(#calc_args);
                #write_def_stmts
            }

            unsafe fn drop_instance(&self, ptr: *mut u8) {
                #drop_def_stmts
            }
        }

        unsafe impl ::slice_struct::ArenaElement for #struct_name #ty_generics {
            type Arena = #arena_name;

            #[inline]
            unsafe fn project_view<'a>(ptr: *const u8, arena: &'a Self::Arena) -> <Self::Arena as ::slice_struct::ArenaDescriptor>::View<'a> {
                let base_ptr = ptr;
                #view_name {
                    #project_fields
                }
            }

            #[inline]
            unsafe fn project_view_mut<'a>(ptr: *mut u8, arena: &'a Self::Arena) -> <Self::Arena as ::slice_struct::ArenaDescriptor>::ViewMut<'a> {
                let base_mut = ptr;
                let base_ptr = ptr as *const u8;
                #view_mut_name {
                    #project_mut_fields
                }
            }
        }
    }
}
