//! proc macro crate for [dstify](https://github.com/jsen-/dstify)

use proc_macro2::TokenStream;
use syn::{
    Attribute, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Type,
    parse_macro_input, parse_quote, spanned::Spanned,
};

#[proc_macro_derive(Dstify)]
pub fn dstify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match inner(input) {
        Ok(output) => output,
        Err(output) => output,
    }
    .into()
}

fn inner(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    ensure_repr_c(&input, &input.attrs)?;

    let a_struct = match &input.data {
        Data::Struct(a_struct) => a_struct,
        Data::Enum(an_enum) => {
            return Err(syn::Error::new(
                an_enum.enum_token.span,
                "`Dstify` cannot be derived for `enums`s",
            )
            .into_compile_error());
        }
        Data::Union(a_union) => {
            return Err(syn::Error::new(
                a_union.union_token.span,
                "`Dstify` cannot be derived for `union`s",
            )
            .into_compile_error());
        }
    };

    let (normal_fields, dst_field_name, dst_field_ty) = match &a_struct.fields {
        Fields::Named(named) => derive_named(&input, named)?,
        Fields::Unnamed(unnamed) => derive_unnamed(&input, unnamed)?,
        Fields::Unit => {
            return Err(syn::Error::new(
                a_struct.struct_token.span,
                "`Dstify` cannot be derived for `unit structs`s",
            )
            .into_compile_error());
        }
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut layouts = Vec::<TokenStream>::new();
    let mut inits = Vec::<TokenStream>::new();
    let args = normal_fields
        .map(|(ident, ty)| {
            layouts.push(parse_quote!(::core::alloc::Layout::new::<#ty>()));
            inits
                .push(parse_quote!(::core::ptr::write(<*mut _>::cast(offsets.get_next()), #ident)));
            parse_quote!(#ident: #ty)
        })
        .collect::<Vec<TokenStream>>();

    let res = parse_quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn init_unsized<R>(#(#args,)* #dst_field_name: &#dst_field_ty) -> R
            where
                R: ::dstify::SmartPointer<Self>
            {
                unsafe {
                    let fat_ptr = ::core::result::Result::unwrap(::dstify::private::alloc::<Self, R, _, _, _>([#(#layouts),*], #dst_field_name, |offsets| {
                        #(#inits;)*
                    }));
                    // this cast must remain here, cannot be done using generics
                    R::cast(fat_ptr as *mut Self)
                }
            }
            fn init_unsized_checked<R>(#(#args,)* #dst_field_name: &#dst_field_ty) -> ::core::result::Result<R, ::core::alloc::LayoutError>
            where
                R: ::dstify::SmartPointer<Self>
            {
                unsafe {
                    let fat_ptr = ::dstify::private::alloc::<Self, R, _, _, _>([#(#layouts),*], #dst_field_name, |offsets| {
                        #(#inits;)*
                    })?;
                    // this cast must remain here, cannot be done using generics
                    Ok(R::cast(fat_ptr as *mut Self))
                }
            }
        }
    };
    Ok(res)
}

type FieldIter<'a> = Box<dyn Iterator<Item = (Ident, &'a Type)> + 'a>;

fn derive_named<'a>(
    input: &'a DeriveInput,
    fields: &'a FieldsNamed,
) -> Result<(FieldIter<'a>, Ident, &'a Type), TokenStream> {
    let mut fields = fields.named.iter().rev();
    let Some(last_field) = fields.next() else {
        return Err(syn::Error::new(
            input.ident.span(),
            "`Dstify` can only be derived for `#[repr(C)]` `struct`s with dynamically-sized last field",
        )
        .into_compile_error());
    };

    let dst_field_ident = last_field
        .ident
        .as_ref()
        .expect("bug: named struct field missing ident")
        .clone();
    let dst_field_ty = &last_field.ty;

    let normal_fields = fields.rev().map(|field| {
        let ident = field
            .ident
            .as_ref()
            .expect("bug: named struct field missing ident");
        let mut name = ident.to_string();
        let name = match name.as_bytes() {
            [b'o', b'f', b'f', b's', b'e', b't', b's', ..] => {
                name.push('_');
                Ident::new(&name, ident.span())
            }
            _ => ident.clone(),
        };
        let ty = &field.ty;
        (name, ty)
    });
    Ok((Box::new(normal_fields), dst_field_ident, dst_field_ty))
}

fn derive_unnamed<'a>(
    input: &'a DeriveInput,
    fields: &'a FieldsUnnamed,
) -> Result<(FieldIter<'a>, Ident, &'a Type), TokenStream> {
    let mut it = fields.unnamed.iter().enumerate().rev();
    let Some(last_field) = it.next() else {
        return Err(syn::Error::new(
            input.ident.span(),
            "`Dstify` can only be derived for `#[repr(C)]` `struct`s with dynamically-sized last field",
        )
        .into_compile_error());
    };

    let dst_field_ident = Ident::new(&format!("f{}", last_field.0), last_field.1.span());
    let dst_field_ty = &last_field.1.ty;

    let normal_fields = it.rev().map(|field| {
        let name = Ident::new(&format!("f{}", field.0), field.1.span());
        let ty = &field.1.ty;
        (name, ty)
    });
    Ok((Box::new(normal_fields), dst_field_ident, dst_field_ty))
}

fn ensure_repr_c(input: &DeriveInput, attrs: &[Attribute]) -> Result<(), TokenStream> {
    let mut found = false;
    for attr in attrs {
        if attr.path().is_ident("repr") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("C") {
                    found = true;
                }
                Ok(())
            })
            .ok();
        }
    }
    if !found {
        return Err(syn::Error::new(
            input.ident.span(),
            "`Dstify` can only be derived for `#[repr(C)]` `struct`s with dynamically-sized last field",
        )
        .into_compile_error());
    }
    Ok(())
}
