#![recursion_limit = "128"]
#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate types;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rundo(args: TokenStream, input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_rundo_attrs(&ast);
    gen.parse().unwrap()
}

fn impl_rundo_attrs(ast: &syn::DeriveInput) -> quote::Tokens {
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

    let op_defineds = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            let op = quote! { <ValueType<#ty> as Rundo>::Op };
            let ops_type = quote!{ Option<Vec<#op>>};
            quote!{ #ident: #ops_type }
        })
        .collect::<Vec<_>>();

    let fileds_ident = fields.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>();

    let reset_method = fileds_ident
        .iter()
        .map(|ident| quote! { self.value.#ident.reset()})
        .collect::<Vec<_>>();
    let ops_method = fileds_ident
        .iter()
        .map(|ident| {
            quote! { #ident: self.value.#ident.change_ops() }
        })
        .collect::<Vec<_>>();

    let m_name = "_".to_owned() + name.as_ref();
    let op_name = "Op".to_owned() + name.as_ref();
    let m_name = syn::Ident::from(m_name);
    let op_name = syn::Ident::from(op_name);

    quote! {
        #sturct_vis struct #op_name { #(#op_defineds), * }

        #sturct_vis struct #m_name { #(#field_defines),* }

        #sturct_vis struct #name {
            value: #m_name,
            dirty: bool,
        }

        impl std::ops::Deref for #name {
            type Target = #m_name;
            fn deref(&self) -> &#m_name { &self.value }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut #m_name {
                  if !self.dirty {
                    self.dirty = true;
                }
                &mut self.value
            }
        }

        // impl Rundo for type
        impl Rundo for #name {

            type Op = #op_name;

            fn dirty(&self) -> bool{
                self.dirty
            }

            fn reset(&mut self) {
                self.dirty = false;
                #(#reset_method ;) *
            }

            fn change_ops(&self)-> Option<std::vec::Vec<#op_name>> {
                match self.dirty {
                    true => Some(vec![ #op_name { #(#ops_method) , *}]),
                    false => None
                }
            }
        }

        /// todo impl a literal macro
        /// # example
        /// #name! { field1: type value, ...}

    }
}
