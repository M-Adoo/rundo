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
            panic!("#[rundo] is only defined for structs, not for enums!");
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("#[rundo] is only defined for structs, not for tuple!");
        }
        syn::Body::Struct(syn::VariantData::Unit) => {
            panic!("#[rundo] is only defined for structs, not for unit!");
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

    let m_name = inner_name(name, "_");
    let op_name = inner_name(name, "Op");
    let literal_macro = impl_stcut_literal(&name, &fields);

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

        #literal_macro
    }
}

fn inner_name(name: &syn::Ident, prefix: &str) -> syn::Ident {
    let in_name = prefix.to_owned() + name.as_ref();
    syn::Ident::from(in_name)
}

fn impl_stcut_literal(name: &syn::Ident, fields: &std::vec::Vec<&syn::Field>) -> quote::Tokens {
    let inner_type = inner_name(name, "_");
    let field_match = fields.into_iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote!{
            // match shorthand literal field constuct like
            // {a, b, c}
            (#ident) => ( ValueType::<#ty>::from(#ident));
            // normal struct literal field construtc like
            // {a: 1, b: 1, c: 1}
            (#ident : $e:tt) => ( ValueType::<#ty>::from($e));
        }
    });

    let field_macro = inner_name(name, "_field");
    let construct = |field_exp: &str| {
        let field_exp = syn::Ident::from(field_exp);
        quote! {
            #name {
                dirty: false,
                value: #inner_type {
                    $($id: #field_macro!(#field_exp)),*
                }
            }
        }
    };
    let shorthand = construct("$id");
    let normal = construct("$id: $e");

    quote! {
        macro_rules! #field_macro {
            // field literal construct match
            #(#field_match)*
        }
        macro_rules! #name {
            ($($id: ident ,) *)  => { #shorthand };
            ($($id: ident), *)  => { #shorthand };
            ($($id: ident : $e: tt ,) *) => { #normal };
            ($($id: ident : $e: tt ), *) => { #normal };
        }
    }
}
