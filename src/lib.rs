// proc macro crate

// for enum, we'd like to generate From impls for each variant

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    print!("{:#?}", input);
    // get the ident
    let ident = &input.ident;
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
                        impl From <#ty> for #ident {
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
    .into()
}
