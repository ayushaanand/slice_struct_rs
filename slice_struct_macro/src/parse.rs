use syn::{Field, Generics, Ident, ItemStruct, Type, Visibility};

pub enum SliceField {
    Flat {
        ident: Ident,
        ty: Type,
        vis: Visibility,
    },
    Arena {
        ident: Ident,
        inner_ty: Type,
        vis: Visibility,
    },
}

impl SliceField {
    pub fn ident(&self) -> &Ident {
        match self {
            SliceField::Flat { ident, .. } => ident,
            SliceField::Arena { ident, .. } => ident,
        }
    }
    pub fn ty(&self) -> Type {
        match self {
            SliceField::Flat { ty, .. } => ty.clone(),
            // Fabricate `ArenaSlice<inner_ty>` to seamlessly work with InlineSlice projections
            SliceField::Arena { inner_ty, .. } => {
                syn::parse_quote!(::slice_struct::ArenaSlice<#inner_ty>)
            }
        }
    }
}

pub struct SliceStructInput {
    pub struct_name: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub sized_fields: Vec<Field>,
    pub slice_fields: Vec<SliceField>,
    pub unpin: bool,
    pub zerocopy: bool,
    pub is_arena: bool,
    pub shared_layout: bool,
}

pub fn parse_slice_struct(mut input: ItemStruct) -> SliceStructInput {
    let struct_name = input.ident.clone();
    let vis = input.vis.clone();
    let generics = input.generics.clone();

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
        let fvis = field.vis.clone();

        if is_slice {
            let mut is_arena = false;
            if let Type::Path(type_path) = &ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "ArenaSlice" {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                slice_fields.push(SliceField::Arena {
                                    ident: ident.clone(),
                                    inner_ty: inner_ty.clone(),
                                    vis: fvis.clone(),
                                });
                                is_arena = true;
                            }
                        }
                    }
                }
            }
            if !is_arena {
                slice_fields.push(SliceField::Flat {
                    ident,
                    ty,
                    vis: fvis,
                });
            }
        } else {
            sized_fields.push(field.clone());
        }
    }

    SliceStructInput {
        struct_name,
        vis,
        generics,
        sized_fields,
        slice_fields,
        unpin: false,
        zerocopy: false,
        is_arena: false,
        shared_layout: false,
    }
}
