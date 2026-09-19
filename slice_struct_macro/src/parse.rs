use syn::{Field, Ident, ItemStruct, Type, Visibility, Generics};

pub struct SliceStructInput {
    pub struct_name: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub sized_fields: Vec<Field>,
    pub slice_fields: Vec<(Ident, Type, Visibility)>,
    pub unpin: bool,
    pub zerocopy: bool,
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
            slice_fields.push((ident, ty, fvis));
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
    }
}
