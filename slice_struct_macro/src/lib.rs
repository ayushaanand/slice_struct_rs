//! Implementation crate for the `#[slice_struct]` procedural macro.
//!
//! **Do not depend on this crate directly.** Use the `slice_struct` crate,
//! which re-exports the macro.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, parse_macro_input};

/// Transform a struct so that fields marked `#[slice]` become inline,
/// variable-length slices packed into a single heap allocation.
///
/// # Syntax
///
/// ```rust,ignore
/// use slice_struct::slice_struct;
///
/// #[slice_struct]
/// pub struct Packet {
///     pub id:      u32,       // plain field — unchanged
///     #[slice] pub payload: u8,  // becomes &[u8] / &mut [u8]
///     #[slice] pub tags:    u32, // becomes &[u32] / &mut [u32]
/// }
/// ```
///
/// To guarantee memory safety, the macro hides the internal layout of your struct. 
/// You interact with the struct exclusively through `.view()` and `.view_mut()`.
///
/// # Generated API
///
/// The macro generates two View structs that inherit the visibility of your fields:
/// - `{StructName}View` — for reading (`&T` and `&[T]`)
/// - `{StructName}ViewMut` — for writing (`&mut T` and `&mut [T]`)
///
/// Two constructors are generated for the whole struct:
///
/// | Constructor | Slice argument | `Clone` required? |
/// |---|---|---|
/// | `new_box_iter` | `impl ExactSizeIterator<Item = T>` | No  |
/// | `new_box_def`  | `(T, usize)` — fill value + length | Yes |
///
/// ### `new_box_iter`
///
/// ```rust,ignore
/// let p = Packet::new_box_iter(
///     42,                           // id
///     [10_u8, 20, 30].into_iter(),  // payload
///     100_u32..104,                 // tags — any ExactSizeIterator
/// );
/// assert_eq!(*p.view().id, 42);
/// assert_eq!(p.view().payload, &[10, 20, 30]);
/// assert_eq!(p.view().tags,    &[100, 101, 102, 103]);
/// ```
///
/// Works with `Vec::into_iter()`, arrays, `Range`, mapped iterators, or
/// anything implementing [`ExactSizeIterator`].  No `Clone` bound is needed
/// and no intermediate buffer is created.
///
/// ### `new_box_def`
///
/// ```rust,ignore
/// // Like vec![0_u8; 8] and vec![0_u32; 4], written directly in-place.
/// let p = Packet::new_box_def(42, (0_u8, 8), (0_u32, 4));
/// assert_eq!(&*p.payload, &[0_u8; 8]);
/// ```
///
/// Requires `Clone` on each slice element type.
///
/// # Limitations
///
/// - Structs must use **named fields** (`struct Foo { field: Type }`).
/// - `#[slice]` fields must come after all plain fields in source order.
/// - `new_box_def` requires `T: Clone` on each slice element type.
/// - The struct is `?Sized` and can only be held behind a pointer (e.g. `Box`).
#[proc_macro_attribute]
pub fn slice_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    let struct_name = &input.ident;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut sized_fields = Vec::new();
    let mut slice_fields = Vec::new();

    for field in &mut input.fields {
        let mut is_slice = false;
        field.attrs.retain(|attr| {
            if attr.path().is_ident("slice") {
                is_slice = true;
                false
            } else {
                true
            }
        });

        let ident = field.ident.clone().unwrap();
        let ty = field.ty.clone();

        let vis = field.vis.clone();

        if is_slice {
            slice_fields.push((ident, ty, vis));
        } else {
            sized_fields.push(field.clone());
        }
    }

    let sized_prefix_ident = format_ident!("{}_SizedPrefix", struct_name);

    // ── Struct field lists ─────────────────────────────────────────────────
    let mut prefix_fields = quote! {};
    let mut actual_fields = quote! {};

    for field in &sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        let ty = &field.ty;
        prefix_fields.extend(quote! { #internal_ident: #ty, });
        actual_fields.extend(quote! { #internal_ident: #ty, });
    }

    for (i, (_, ty, _)) in slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        prefix_fields.extend(quote! { #align_ident: [#ty; 0], });
        actual_fields.extend(quote! { #align_ident: [#ty; 0], });
    }

    for (ident, ty, _) in &slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        prefix_fields.extend(quote! { #internal_ident: ::slice_struct::SliceHandle<#ty>, });
        actual_fields.extend(quote! { #internal_ident: ::slice_struct::SliceHandle<#ty>, });
    }

    let data_field = quote! {
        __pin: ::core::marker::PhantomPinned,
        #[doc(hidden)]
        pub __data_tail: [::core::mem::MaybeUninit<u8>]
    };

    // ── __layout: compute Layout + byte offsets for each slice ─────────────
    let len_idents_only: Vec<_> = slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_len", id))
        .collect();
    let offset_idents: Vec<_> = slice_fields
        .iter()
        .map(|(id, _, _)| format_ident!("{}_offset", id))
        .collect();
    let layout_ret_types: Vec<_> = slice_fields.iter().map(|_| quote!(usize)).collect();

    let mut layout_stmts = quote! {
        let layout = ::std::alloc::Layout::new::<#sized_prefix_ident #ty_generics>();
    };
    for (ident, ty, _) in &slice_fields {
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

    let mut view_generics = input.generics.clone();
    view_generics.params.insert(0, ::syn::parse_quote!('__a));
    let (view_impl_generics, view_ty_generics, _) = view_generics.split_for_impl();

    let view_ident = format_ident!("{}View", struct_name);
    let view_mut_ident = format_ident!("{}ViewMut", struct_name);

    let mut view_fields = quote! {};
    let mut view_init = quote! {};
    
    let mut view_mut_fields = quote! {};
    let mut view_mut_init = quote! {};

    for field in &sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        let ty = &field.ty;
        let fvis = &field.vis;
        
        view_fields.extend(quote! { #fvis #ident: &'__a #ty, });
        view_init.extend(quote! { #ident: &this.#internal_ident, });
        
        view_mut_fields.extend(quote! { #fvis #ident: &'__a mut #ty, });
        view_mut_init.extend(quote! { #ident: &mut this.#internal_ident, });
    }

    for (ident, ty, fvis) in &slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        
        view_fields.extend(quote! { #fvis #ident: &'__a [#ty], });
        view_init.extend(quote! { #ident: &this.#internal_ident, });
        
        view_mut_fields.extend(quote! { #fvis #ident: ::slice_struct::SliceBorrow<'__a, #ty>, });
        view_mut_init.extend(quote! {
            #ident: ::slice_struct::SliceBorrow::__from_handle(
                ::core::pin::Pin::new_unchecked(&mut this.#internal_ident)
            ),
        });
    }

    let projection_code = quote! {
        #[doc = "An immutable view into the fields of the struct, providing `&T` for normal fields and `&[T]` for slice fields."]
        #vis struct #view_ident #view_impl_generics #where_clause {
            #view_fields
        }

        #[doc = "A mutable view into the fields of the struct, providing `&mut T` for normal fields and `SliceBorrow` (which acts as `&mut [T]`) for slice fields.\n\nThis struct enables safe disjoint borrowing of multiple slices simultaneously."]
        #vis struct #view_mut_ident #view_impl_generics #where_clause {
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
    };

    // ── Shared helpers ─────────────────────────────────────────────────────
    let mut sized_args = quote! {};
    for field in &sized_fields {
        let ident = &field.ident;
        let ty = &field.ty;
        sized_args.extend(quote! { #ident: #ty, });
    }

    prefix_fields.extend(quote! {
        __pin: ::core::marker::PhantomPinned,
    });

    let mut prefix_init = quote! {};
    for field in &sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        prefix_init.extend(quote! { #internal_ident: #ident, });
    }
    for (i, _) in slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        prefix_init.extend(quote! { #align_ident: [], });
    }
    for (ident, ty, _) in &slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        let len_ident = format_ident!("{}_len", ident);
        let offset_ident = format_ident!("{}_offset", ident);
        prefix_init.extend(quote! { 
            #internal_ident: ::slice_struct::SliceHandle::__new_unchecked(ptr.add(#offset_ident).cast::<#ty>(), #len_ident), 
        });
    }
    prefix_init.extend(quote! {
        __pin: ::core::marker::PhantomPinned,
    });

    let alloc_block = quote! {
        let (layout, #(#offset_idents),*) = Self::__layout(#(#len_idents_only),*);
        let ptr: *mut u8 = ::std::alloc::alloc(layout);
        if ptr.is_null() { ::std::alloc::handle_alloc_error(layout); }
        let sized_ptr = ptr.cast::<#sized_prefix_ident #ty_generics>();
        ::core::ptr::write(sized_ptr, #sized_prefix_ident { #prefix_init });
    };

    let seal_block = quote! {
        let data_len = layout.size()
            - ::core::mem::size_of::<#sized_prefix_ident #ty_generics>();
        let fat_ptr =
            ::core::ptr::slice_from_raw_parts_mut(ptr.cast::<()>(), data_len) as *mut Self;
        ::std::boxed::Box::from_raw(fat_ptr)
    };

    // ── new_box_iter ───────────────────────────────────────────────────────
    let mut iter_args = sized_args.clone();
    let mut iter_param_idents = vec![];
    for (ident, ty, _) in &slice_fields {
        let iter_param = format_ident!("{}_iter", ident);
        iter_args.extend(quote! { #iter_param: impl ::core::iter::ExactSizeIterator<Item = #ty>, });
        iter_param_idents.push(iter_param);
    }

    let mut iter_len_vars = quote! {};
    for (i, (ident, _, _)) in slice_fields.iter().enumerate() {
        let len_ident = format_ident!("{}_len", ident);
        let iter_param = &iter_param_idents[i];
        iter_len_vars.extend(quote! { let #len_ident = #iter_param.len(); });
    }

    let mut iter_write_slices = quote! {};
    for (i, (_, ty, _)) in slice_fields.iter().enumerate() {
        let o_ident = &offset_idents[i];
        let iter_param = &iter_param_idents[i];
        let len_ident = format_ident!("{}_len", slice_fields[i].0);
        iter_write_slices.extend(quote! {
            let field_ptr = ptr.add(#o_ident).cast::<#ty>();
            // Drop guard: if the iterator panics mid-write, drop already-written elements.
            let mut written = 0usize;
            let guard = ::slice_struct::__DropGuard::new(field_ptr, &mut written);
            let mut iter = #iter_param.into_iter();
            for j in 0..#len_ident {
                let item = iter.next().expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                written += 1;
            }
            ::core::mem::forget(guard);
        });
    }

    // ── new_box_def ────────────────────────────────────────────────────────
    let mut def_args = sized_args.clone();
    let mut def_val_idents = vec![];
    for (ident, ty, _) in &slice_fields {
        let val_ident = format_ident!("{}_def", ident);
        def_args.extend(quote! { #val_ident: (#ty, usize), });
        def_val_idents.push(val_ident);
    }

    let mut def_len_vars = quote! {};
    for (i, (ident, _, _)) in slice_fields.iter().enumerate() {
        let len_ident = format_ident!("{}_len", ident);
        let val_ident = &def_val_idents[i];
        def_len_vars.extend(quote! { let #len_ident = #val_ident.1; });
    }

    let mut def_write_slices = quote! {};
    let mut def_where_bounds = vec![];
    for (i, (_, ty, _)) in slice_fields.iter().enumerate() {
        let o_ident = &offset_idents[i];
        let val_ident = &def_val_idents[i];

        def_where_bounds.push(quote! { #ty: ::core::clone::Clone });

        def_write_slices.extend(quote! {
            let (def_val, def_len) = #val_ident;
            let field_ptr = ptr.add(#o_ident).cast::<#ty>();
            if def_len > 0 {
                for j in 0..def_len - 1 {
                    ::core::ptr::write(field_ptr.add(j), ::core::clone::Clone::clone(&def_val));
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
        });
    }

    let expanded = quote! {
        #[repr(C)]
        #[doc(hidden)]
        #vis struct #sized_prefix_ident #impl_generics #where_clause {
            #prefix_fields
        }

        #[repr(C)]
        #vis struct #struct_name #impl_generics #where_clause {
            #actual_fields
            #data_field
        }

        #projection_code

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc(hidden)]
            #[inline]
            fn __layout(#(#len_idents_only: usize),*)
                -> (::std::alloc::Layout, #(#layout_ret_types),*)
            {
                #layout_stmts
            }

            /// Construct a `Box<Self>` by draining one [`ExactSizeIterator`]
            /// per slice field.
            ///
            /// Elements are moved directly into the allocation — no
            /// intermediate buffer is created and no `Clone` bound is required.
            ///
            /// # Panics
            ///
            /// - Panics if the allocator returns null (out of memory).
            /// - Panics if any of the provided `ExactSizeIterator`s yield fewer elements than their `.len()` claims, 
            ///   to prevent reading uninitialized memory.
            ///
            /// # Example
            /// 
            /// ```rust,ignore
            /// // Assuming a struct defined as:
            /// // #[slice_struct] struct Packet { id: u32, #[slice] data: u8 }
            /// 
            /// let p = Packet::new_box_iter(
            ///     42,                       // id: u32
            ///     vec![1, 2, 3].into_iter() // data: impl ExactSizeIterator<Item = u8>
            /// );
            /// ```
            #vis fn new_box_iter(#iter_args) -> ::core::pin::Pin<::std::boxed::Box<Self>> {
                #iter_len_vars
                unsafe {
                    #alloc_block
                    #iter_write_slices
                    ::core::pin::Pin::new_unchecked({ #seal_block })
                }
            }

            /// Construct a `Box<Self>` from `(value, length)` pairs —
            /// analogous to `vec![val; len]` for each slice field.
            ///
            /// Requires `Clone` on each slice element type.
            ///
            /// # Panics
            ///
            /// Panics if the allocator returns null (out of memory).
            ///
            /// # Example
            /// 
            /// ```rust,ignore
            /// // Assuming a struct defined as:
            /// // #[slice_struct] struct Packet { id: u32, #[slice] data: u8 }
            /// 
            /// let p = Packet::new_box_def(
            ///     42,       // id: u32
            ///     (0, 100)  // data: (u8, usize)
            /// );
            /// ```
            #vis fn new_box_def(#def_args) -> ::core::pin::Pin<::std::boxed::Box<Self>>
            where
                #(#def_where_bounds),*
            {
                #def_len_vars
                unsafe {
                    #alloc_block
                    #def_write_slices
                    ::core::pin::Pin::new_unchecked({ #seal_block })
                }
            }
        }
    };

    expanded.into()
}
