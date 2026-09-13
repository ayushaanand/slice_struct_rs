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
/// Fields **without** `#[slice]` are stored as-is and accessed directly by
/// name.  Fields **with** `#[slice]` become variable-length sequences accessed
/// through generated methods.
///
/// # Generated API
///
/// For each `#[slice] foo: T` field the macro generates:
///
/// ```rust,ignore
/// fn foo(&self)         -> &[T]
/// fn foo_mut(&mut self) -> &mut [T]
/// ```
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
/// assert_eq!(p.payload(), &[10, 20, 30]);
/// assert_eq!(p.tags(),    &[100, 101, 102, 103]);
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
/// assert_eq!(p.payload(), &[0_u8; 8]);
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

        if is_slice {
            slice_fields.push((ident, ty));
        } else {
            sized_fields.push(field.clone());
        }
    }

    let sized_prefix_ident = format_ident!("{}_SizedPrefix", struct_name);

    // ── Struct field lists ─────────────────────────────────────────────────
    let mut prefix_fields = quote! {};
    let mut actual_fields = quote! {};

    for field in &sized_fields {
        let ident = &field.ident;
        let ty = &field.ty;
        let fvis = &field.vis;
        prefix_fields.extend(quote! { #fvis #ident: #ty, });
        actual_fields.extend(quote! { #fvis #ident: #ty, });
    }

    for (ident, ty) in &slice_fields {
        let len_ident = format_ident!("__{}_len", ident);
        let align_ident = format_ident!("__{}_align", ident);
        prefix_fields.extend(quote! { #len_ident: usize, #align_ident: [#ty; 0], });
        actual_fields.extend(quote! { #len_ident: usize, #align_ident: [#ty; 0], });
    }

    let data_field = quote! { __data: [::core::mem::MaybeUninit<u8>] };

    // ── __layout: compute Layout + byte offsets for each slice ─────────────
    let len_idents_only: Vec<_> = slice_fields
        .iter()
        .map(|(id, _)| format_ident!("{}_len", id))
        .collect();
    let offset_idents: Vec<_> = slice_fields
        .iter()
        .map(|(id, _)| format_ident!("{}_offset", id))
        .collect();
    let layout_ret_types: Vec<_> = slice_fields.iter().map(|_| quote!(usize)).collect();

    let mut layout_stmts = quote! {
        let layout = ::std::alloc::Layout::new::<#sized_prefix_ident #ty_generics>();
    };
    for (ident, ty) in &slice_fields {
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

    // ── Slice getters / setters ────────────────────────────────────────────
    let mut getters = quote! {};
    for (i, (ident, ty)) in slice_fields.iter().enumerate() {
        let len_ident = format_ident!("__{}_len", ident);
        let mut offset_vars = vec![quote!(_); slice_fields.len()];
        offset_vars[i] = quote!(offset);

        let len_args: Vec<_> = slice_fields
            .iter()
            .map(|(id, _)| {
                let l = format_ident!("__{}_len", id);
                quote!(self.#l)
            })
            .collect();

        let mut_ident = format_ident!("{}_mut", ident);
        getters.extend(quote! {
            /// Returns a shared reference to the `#ident` slice.
            #vis fn #ident(&self) -> &[#ty] {
                let (_, #(#offset_vars),*) = Self::__layout(#(#len_args),*);
                let ptr = self as *const Self as *const u8;
                unsafe {
                    let field_ptr = ptr.add(offset).cast::<#ty>();
                    ::core::slice::from_raw_parts(field_ptr, self.#len_ident)
                }
            }

            /// Returns a mutable reference to the `#ident` slice.
            #vis fn #mut_ident(&mut self) -> &mut [#ty] {
                let (_, #(#offset_vars),*) = Self::__layout(#(#len_args),*);
                let ptr = self as *mut Self as *mut u8;
                unsafe {
                    let field_ptr = ptr.add(offset).cast::<#ty>();
                    ::core::slice::from_raw_parts_mut(field_ptr, self.#len_ident)
                }
            }
        });
    }

    // ── Shared helpers ─────────────────────────────────────────────────────
    let mut sized_args = quote! {};
    for field in &sized_fields {
        let ident = &field.ident;
        let ty = &field.ty;
        sized_args.extend(quote! { #ident: #ty, });
    }

    let mut prefix_init = quote! {};
    for field in &sized_fields {
        let ident = &field.ident;
        prefix_init.extend(quote! { #ident, });
    }
    for (ident, _) in &slice_fields {
        let len_struct_ident = format_ident!("__{}_len", ident);
        let align_ident = format_ident!("__{}_align", ident);
        let len_ident = format_ident!("{}_len", ident);
        prefix_init.extend(quote! { #len_struct_ident: #len_ident, #align_ident: [], });
    }

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
    for (ident, ty) in &slice_fields {
        let iter_param = format_ident!("{}_iter", ident);
        iter_args.extend(quote! { #iter_param: impl ::core::iter::ExactSizeIterator<Item = #ty>, });
        iter_param_idents.push(iter_param);
    }

    let mut iter_len_vars = quote! {};
    for (i, (ident, _)) in slice_fields.iter().enumerate() {
        let len_ident = format_ident!("{}_len", ident);
        let iter_param = &iter_param_idents[i];
        iter_len_vars.extend(quote! { let #len_ident = #iter_param.len(); });
    }

    let mut iter_write_slices = quote! {};
    for (i, (_, ty)) in slice_fields.iter().enumerate() {
        let o_ident = &offset_idents[i];
        let iter_param = &iter_param_idents[i];
        iter_write_slices.extend(quote! {
            let field_ptr = ptr.add(#o_ident).cast::<#ty>();
            for (j, item) in #iter_param.enumerate() {
                ::core::ptr::write(field_ptr.add(j), item);
            }
        });
    }

    // ── new_box_def ────────────────────────────────────────────────────────
    let mut def_args = sized_args.clone();
    let mut def_val_idents = vec![];
    for (ident, ty) in &slice_fields {
        let val_ident = format_ident!("{}_def", ident);
        def_args.extend(quote! { #val_ident: (#ty, usize), });
        def_val_idents.push(val_ident);
    }

    let mut def_len_vars = quote! {};
    for (i, (ident, _)) in slice_fields.iter().enumerate() {
        let len_ident = format_ident!("{}_len", ident);
        let val_ident = &def_val_idents[i];
        def_len_vars.extend(quote! { let #len_ident = #val_ident.1; });
    }

    let mut def_write_slices = quote! {};
    for (i, (_, ty)) in slice_fields.iter().enumerate() {
        let o_ident = &offset_idents[i];
        let val_ident = &def_val_idents[i];
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

    // ── Drop ───────────────────────────────────────────────────────────────
    let mut drop_stmts = quote! {};
    for (ident, _) in &slice_fields {
        let mut_ident = format_ident!("{}_mut", ident);
        drop_stmts.extend(quote! {
            unsafe { ::core::ptr::drop_in_place(self.#mut_ident()); }
        });
    }

    // ── Emit ───────────────────────────────────────────────────────────────
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

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc(hidden)]
            #[inline]
            fn __layout(#(#len_idents_only: usize),*)
                -> (::std::alloc::Layout, #(#layout_ret_types),*)
            {
                #layout_stmts
            }

            #getters

            /// Construct a `Box<Self>` by draining one [`ExactSizeIterator`]
            /// per slice field.
            ///
            /// Elements are moved directly into the allocation — no
            /// intermediate buffer is created and no `Clone` bound is required.
            ///
            /// # Panics
            ///
            /// Panics if the allocator returns null (out of memory).
            #vis fn new_box_iter(#iter_args) -> ::std::boxed::Box<Self> {
                #iter_len_vars
                unsafe {
                    #alloc_block
                    #iter_write_slices
                    #seal_block
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
            #vis fn new_box_def(#def_args) -> ::std::boxed::Box<Self> {
                #def_len_vars
                unsafe {
                    #alloc_block
                    #def_write_slices
                    #seal_block
                }
            }
        }

        impl #impl_generics ::core::ops::Drop for #struct_name #ty_generics #where_clause {
            fn drop(&mut self) {
                #drop_stmts
            }
        }
    };

    expanded.into()
}
