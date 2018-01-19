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
    let ast = syn::parse(input).expect("Well, can't parse the code, maybe some syntax error!");
    impl_rundo_attrs(&ast).into()
}

fn impl_rundo_attrs(item: &syn::Item) -> quote::Tokens {
    if let &syn::Item::Struct(ref s) = item {
        let name = &s.ident;
        let sturct_vis = &s.vis;
        let fields = match s.fields {
            syn::Fields::Named(ref fs) => fs,
            _ => panic!("Rundo not support for Tuple sturct or unit struct now"),
        };

        let field_defines = fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref();
                let ty = &field.ty;
                let vis = &field.vis;
                quote!{ #vis #ident: ValueType<#ty> }
            })
            .collect::<Vec<_>>();

        let op_defineds = fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref();
                let ty = &field.ty;
                let op = quote! { <ValueType<#ty> as Rundo>::Op };
                let ops_type = quote!{ Option<Vec<#op>>};
                quote!{ #ident: #ops_type }
            })
            .collect::<Vec<_>>();

        let fileds_ident = fields
            .named
            .iter()
            .map(|f| f.ident.as_ref())
            .collect::<Vec<_>>();

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
        let literal_macro = impl_stcut_literal(&name, fields);
        println!("{:?}", literal_macro);
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
    } else {
        panic!("#[rundo] is only support for structs now!");
    }
}

fn inner_name(name: &syn::Ident, prefix: &str) -> syn::Ident {
    let in_name = prefix.to_owned() + name.as_ref();
    syn::Ident::from(in_name)
}

fn impl_stcut_literal(name: &syn::Ident, fields: &syn::FieldsNamed) -> quote::Tokens {
    let inner_type = inner_name(name, "_");
    let field_match = fields.named.iter().map(|field| {
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

    let field_macro = inner_name(name, "_field_");
    let construct = |field_exp| {
        quote! {
            #name {
                dirty: false,
                value: #inner_type {
                    $($id: #field_macro!(#field_exp)),*
                }
            }
        }
    };

    let shorthand = construct(quote! {$id});
    let normal = construct(quote!{$id: $e});

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
