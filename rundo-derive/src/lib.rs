#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate types;

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
    let sturct_vis = &ast.vis;
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
            let vis = &field.vis;
            quote!{ #vis #ident: ValueType<#ty> }
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
    let op_name = "Op".to_owned() + name.as_ref();
    let m_name = syn::Ident::from(m_name);
    let r_name = syn::Ident::from(r_name);
    let op_name = syn::Ident::from(op_name);

    quote! {
        #sturct_vis struct #op_name;

        #sturct_vis struct #m_name { #(#field_defines),* }

        #sturct_vis struct #r_name {
            value: #m_name,
            pub ops: Option<std::vec::Vec<#op_name>>,
            dirty: bool,
        }

        impl std::ops::Deref for #r_name {
            type Target = #m_name;
            fn deref(&self) -> &#m_name { &self.value }
        }

        impl std::ops::DerefMut for #r_name {
            fn deref_mut(&mut self) -> &mut #m_name {
                  if !self.dirty {
                    self.dirty = true;
                }
                &mut self.value
            }
        }

        impl std::convert::From<#name> for #r_name {
            fn from(from: #name) -> Self {
                let v = #m_name {
                    #(#fromed ,) *
                };

                #r_name {
                    dirty: false,
                    value: v,
                    ops: None,
                }
            }
        }

        impl Rundo for #r_name {

            type Op = #op_name;

            fn dirty(&self) -> bool{
                self.dirty
            }

            fn reset(&mut self) {
                self.dirty = false;
            }

            fn change_ops(&self)-> Option<std::vec::Vec<#op_name>> {
                unimplemented!();
            }
        }
    }
}
