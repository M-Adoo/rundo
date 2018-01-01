extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate types;

use std::convert::From;
use proc_macro::TokenStream;

#[proc_macro_derive(Rundo)]
pub fn rundo(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_rundo_derive(&ast);
    gen.parse().unwrap()
}

fn impl_rundo_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = match ast.body {
        syn::Body::Enum(_) => {
            panic!("#[derive(Rundo)] is only defined for structs, not for enums!");
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("#[derive(Rundo)] is only defined for structs, not for tuple!");
        }
        syn::Body::Struct(syn::VariantData::Unit) => {
            panic!("#[derive(Rundo)] is only defined for structs, not for unit!");
        }
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => body.iter().collect::<Vec<_>>(),
    };

    let field_defines = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            quote!{ #ident: ValueType<#ty> }
        })
        .collect::<Vec<_>>();

    let fromed = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            quote!{ #ident: ValueType::<#ty>::from(from.#ident)}
        })
        .collect::<Vec<_>>();

    let m_name = "M_".to_owned() + name.as_ref();
    let r_name = "R_".to_owned() + name.as_ref();
    let m_name = syn::Ident::from(m_name);
    let r_name = syn::Ident::from(r_name);

    let tokens = quote! {
           pub struct #m_name { #(#field_defines),* }

           pub type #r_name = OpType<#m_name>;

           impl From<#name> for #r_name {
                fn from(from: #name) -> Self {
                    let v = #m_name {
                        #(#fromed ,) *
                    };
                    OpType::from(v)
                }
            }
    };
    println!("{}", tokens);
    tokens
}
