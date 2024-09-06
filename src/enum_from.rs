use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    print!("{:#?}", input);
    // get the ident
    let ident = &input.ident;
    // get generics
    let generics = &input.generics;
    // get enum variants
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support single tuple variant
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = &fields.unnamed.first().unwrap();
                    let ty = &field.ty;
                    quote! {
                        impl #generics From <#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}
