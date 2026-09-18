use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::parse::SliceStructInput;

pub fn generate(input: &SliceStructInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let sized_prefix_ident = format_ident!("{}_SizedPrefix", struct_name);
    
    let init_iter_ident = format_ident!("__{}InitIter", struct_name);
    let init_def_ident = format_ident!("__{}InitDef", struct_name);

    let mode = if input.unpin {
        quote! { ::slice_struct::RelativeMode }
    } else {
        quote! { ::slice_struct::AbsoluteMode }
    };
    
    // Shared prefix init logic
    let mut prefix_init = quote! {};
    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let internal_ident = format_ident!("__{}", ident);
        prefix_init.extend(quote! { #internal_ident: self.#ident, });
    }
    for (i, _) in input.slice_fields.iter().enumerate() {
        let align_ident = format_ident!("__align_{}", i);
        prefix_init.extend(quote! { #align_ident: [], });
    }
    
    let mut fixup_len_args = quote! {};
    for (ident, _, _) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        fixup_len_args.extend(quote! { (*ptr).#internal_ident.len, });
    }
    
    let offset_vars: Vec<_> = input.slice_fields.iter().map(|(id, _, _)| format_ident!("{}_offset", id)).collect();
    
    let mut fixup_write_pointers = quote! {};
    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        let internal_ident = format_ident!("__{}", ident);
        let offset = &offset_vars[i];
        fixup_write_pointers.extend(quote! {
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((*ptr).#internal_ident.ptr_data),
                <#mode as ::slice_struct::AddressingMode>::store::<<#ty as ::slice_struct::InlineSlice>::Element>(ptr as *mut u8, #offset)
            );
        });
    }

    let turbofish = ty_generics.as_turbofish();
    
    let fixup_impl = quote! {
        unsafe fn fixup(ptr: *mut #struct_name #ty_generics) {
            let (_, #(#offset_vars),*) = #struct_name #turbofish::__layout(#fixup_len_args);
            #fixup_write_pointers
        }
        
        fn make_fat_ptr(&self, base: *mut u8) -> *mut #struct_name #ty_generics {
            let data_len = self.layout().size() - ::core::mem::size_of::<#sized_prefix_ident #ty_generics>();
            ::core::ptr::slice_from_raw_parts_mut(base.cast::<()>(), data_len) as *mut _
        }
    };
    
    // Iter Init
    let mut iter_generics = input.generics.clone();
    let mut iter_type_params = Vec::new();
    let mut iter_fields = quote! {};
    let mut iter_bounds = quote! {};
    let mut iter_len_vars = quote! {};
    let mut iter_len_args = quote! {};
    let mut iter_write_slices = quote! {};
    let mut iter_fn_args = quote! {};
    let mut iter_fn_init = quote! {};
    
    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        iter_fields.extend(quote! { pub #ident: #ty, });
        iter_fn_args.extend(quote! { #ident: #ty, });
        iter_fn_init.extend(quote! { #ident, });
    }
    
    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        let generic_ident = format_ident!("__I{}", i);
        iter_generics.params.push(::syn::parse_quote!(#generic_ident));
        iter_type_params.push(generic_ident.clone());
        iter_fields.extend(quote! { pub #ident: #generic_ident, });
        iter_bounds.extend(quote! { #generic_ident: ::core::iter::ExactSizeIterator<Item = <#ty as ::slice_struct::InlineSlice>::Element>, });
        
        iter_fn_args.extend(quote! { mut #ident: #generic_ident, });
        iter_fn_init.extend(quote! { #ident, });
        
        let len_ident = format_ident!("{}_len", ident);
        iter_len_vars.extend(quote! { let #len_ident = self.#ident.len(); });
        iter_len_args.extend(quote! { #len_ident, });
        
        let offset = &offset_vars[i];
        iter_write_slices.extend(quote! {
            let field_ptr = ptr.add(#offset).cast::<<#ty as ::slice_struct::InlineSlice>::Element>();
            let mut guard = ::slice_struct::__DropGuard::new(field_ptr);
            let mut iter = self.#ident.into_iter();
            for j in 0..#len_ident {
                let item = iter.next().expect("ExactSizeIterator yielded fewer elements than its len()");
                ::core::ptr::write(field_ptr.add(j), item);
                guard.written += 1;
            }
            ::core::mem::forget(guard);
        });
    }
    
    let (iter_impl_generics, iter_ty_generics, _) = iter_generics.split_for_impl();
    
    let mut prefix_init_iter = prefix_init.clone();
    for (ident, ty, _) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);
        let len_ident = format_ident!("{}_len", ident);
        prefix_init_iter.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::init_state(),
            #internal_ident: ::slice_struct::SliceHandle {
                ptr_data: <#mode as ::slice_struct::AddressingMode>::dummy::<<#ty as ::slice_struct::InlineSlice>::Element>(),
                len: #len_ident,
                _marker: ::core::marker::PhantomData,
            },
        });
    }
    prefix_init_iter.extend(quote! { __pin: <#mode as ::slice_struct::AddressingMode>::MARKER_INIT, });

    let iter_def = quote! {
        #[doc(hidden)]
        pub struct #init_iter_ident #iter_generics {
            #iter_fields
            _marker: ::core::marker::PhantomData<#struct_name #ty_generics>,
        }
        
        unsafe impl #iter_impl_generics ::slice_struct::SliceInit<#struct_name #ty_generics> for #init_iter_ident #iter_ty_generics
        where #iter_bounds
        {
            fn layout(&self) -> ::std::alloc::Layout {
                #iter_len_vars
                #struct_name #turbofish::__layout(#iter_len_args).0
            }
            
            #fixup_impl
            
            unsafe fn write_data(mut self, ptr: *mut u8) -> ::slice_struct::OwnedDst<#struct_name #ty_generics> {
                #iter_len_vars
                let (layout, #(#offset_vars),*) = #struct_name #turbofish::__layout(#iter_len_args);
                let fat_ptr = self.make_fat_ptr(ptr);
                
                let sized_ptr = ptr.cast::<#sized_prefix_ident #ty_generics>();
                ::core::ptr::write(sized_ptr, #sized_prefix_ident { #prefix_init_iter });
                
                #iter_write_slices
                
                ::slice_struct::OwnedDst {
                    ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                    layout
                }
            }
        }
    };
    
    // Def Init
    let mut def_bounds = quote! {};
    let mut def_len_vars = quote! {};
    let mut def_len_args = quote! {};
    let mut def_write_slices = quote! {};
    let mut def_fn_args = quote! {};
    let mut def_fn_init = quote! {};
    let mut def_fields = quote! {};
    
    for field in &input.sized_fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        def_fields.extend(quote! { pub #ident: #ty, });
        def_fn_args.extend(quote! { #ident: #ty, });
        def_fn_init.extend(quote! { #ident, });
    }
    
    for (i, (ident, ty, _)) in input.slice_fields.iter().enumerate() {
        def_bounds.extend(quote! { <#ty as ::slice_struct::InlineSlice>::Element: ::core::clone::Clone, });
        def_fields.extend(quote! { pub #ident: (<#ty as ::slice_struct::InlineSlice>::Element, usize), });
        
        def_fn_args.extend(quote! { #ident: (<#ty as ::slice_struct::InlineSlice>::Element, usize), });
        def_fn_init.extend(quote! { #ident, });
        
        let len_ident = format_ident!("{}_len", ident);
        def_len_vars.extend(quote! { let #len_ident = self.#ident.1; });
        def_len_args.extend(quote! { #len_ident, });
        
        let offset = &offset_vars[i];
        def_write_slices.extend(quote! {
            let def_len = self.#ident.1;
            let field_ptr = ptr.add(#offset).cast::<<#ty as ::slice_struct::InlineSlice>::Element>();
            if def_len > 0 {
                let def_val = self.#ident.0;
                for j in 0..def_len - 1 {
                    ::core::ptr::write(field_ptr.add(j), ::core::clone::Clone::clone(&def_val));
                }
                ::core::ptr::write(field_ptr.add(def_len - 1), def_val);
            }
        });
    }
    
    let mut prefix_init_def = prefix_init.clone();
    for (ident, ty, _) in &input.slice_fields {
        let internal_ident = format_ident!("__{}", ident);
        let state_ident = format_ident!("__{}_state", ident);
        let len_ident = format_ident!("{}_len", ident);
        prefix_init_def.extend(quote! {
            #state_ident: <#ty as ::slice_struct::InlineSlice>::init_state(),
            #internal_ident: ::slice_struct::SliceHandle {
                ptr_data: <#mode as ::slice_struct::AddressingMode>::dummy::<<#ty as ::slice_struct::InlineSlice>::Element>(),
                len: #len_ident,
                _marker: ::core::marker::PhantomData,
            },
        });
    }
    prefix_init_def.extend(quote! { __pin: <#mode as ::slice_struct::AddressingMode>::MARKER_INIT, });
    
    let mut def_generics = input.generics.clone();
    for (_, ty, _) in &input.slice_fields {
        def_generics.make_where_clause().predicates.push(::syn::parse_quote!(<#ty as ::slice_struct::InlineSlice>::Element: ::core::clone::Clone));
    }
    let (_, _, def_where_clause_with_clone) = def_generics.split_for_impl();

    let def_def = quote! {
        #[doc(hidden)]
        pub struct #init_def_ident #def_generics {
            #def_fields
            _marker: ::core::marker::PhantomData<#struct_name #ty_generics>,
        }
        
        unsafe impl #impl_generics ::slice_struct::SliceInit<#struct_name #ty_generics> for #init_def_ident #ty_generics
        #def_where_clause_with_clone
        {
            fn layout(&self) -> ::std::alloc::Layout {
                #def_len_vars
                #struct_name #turbofish::__layout(#def_len_args).0
            }
            
            #fixup_impl
            
            unsafe fn write_data(self, ptr: *mut u8) -> ::slice_struct::OwnedDst<#struct_name #ty_generics> {
                #def_len_vars
                let (layout, #(#offset_vars),*) = #struct_name #turbofish::__layout(#def_len_args);
                let fat_ptr = self.make_fat_ptr(ptr);
                
                let sized_ptr = ptr.cast::<#sized_prefix_ident #ty_generics>();
                ::core::ptr::write(sized_ptr, #sized_prefix_ident { #prefix_init_def });
                
                #def_write_slices
                
                ::slice_struct::OwnedDst {
                    ptr: ::core::ptr::NonNull::new_unchecked(fat_ptr),
                    layout
                }
            }
        }
    };
    
    let iter_type_params_list: Vec<_> = iter_type_params.clone();

    quote! {
        #iter_def
        #def_def
        
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = "Initialize this struct from iterators, returning a builder."]
            #vis fn init_iter<#(#iter_type_params_list),*>(#iter_fn_args) -> ::slice_struct::SliceBuilder<Self, #init_iter_ident #iter_ty_generics>
            where #iter_bounds
            {
                ::slice_struct::SliceBuilder::new(#init_iter_ident { #iter_fn_init _marker: ::core::marker::PhantomData })
            }
            
            #[doc = "Initialize this struct from cloned values, returning a builder."]
            #vis fn init_def(#def_fn_args) -> ::slice_struct::SliceBuilder<Self, #init_def_ident #ty_generics>
            #def_where_clause_with_clone
            {
                ::slice_struct::SliceBuilder::new(#init_def_ident { #def_fn_init _marker: ::core::marker::PhantomData })
            }
        }
    }
}
