use quote;
use syn;
use rundo_types::IMPLED_RUNDO;
use syn::token::Comma;
use syn::punctuated::Punctuated;
use syn::{Field, Fields};
use syn::NestedMeta::Meta;
use syn::Meta::{List, Word};

pub fn prefix_ident(ident: &syn::Ident, prefix: &str) -> syn::Ident {
    let in_name = prefix.to_owned() + ident.as_ref();
    syn::Ident::from(in_name)
}

pub trait RundoStruct {
    fn op_name(&self) -> syn::Ident;
    fn struct_def(&self) -> quote::Tokens;
    fn op_struct_def(&self) -> quote::Tokens;
    fn impl_rundo(&self) -> quote::Tokens;
}

impl RundoStruct for syn::ItemStruct {
    fn op_name(&self) -> syn::Ident {
        prefix_ident(&self.ident, "Op")
    }

    fn struct_def(&self) -> quote::Tokens {
        let vis = &self.vis;
        let name = &self.ident;
        let fields_def = self.fields.fields_def();
        quote! {
            #vis struct #name { #fields_def }
        }
    }

    fn op_struct_def(&self) -> quote::Tokens {
        let vis = &self.vis;
        let name = self.op_name();
        let ops_def = self.fields.op_def();
        quote! {
            #[derive(Debug)]
            #vis struct #name {
                 #ops_def
            }
        }
    }

    fn impl_rundo(&self) -> quote::Tokens {
        let name = &self.ident;
        let op_name = &self.op_name();
        let reset_impl = self.fields.reset_method();
        let ops_impl = self.fields.op_method();
        let back_impl = self.fields.back_method();
        let forward_impl = self.fields.forward_method();
        let dirty_method = self.fields.dirty_method();
        quote! {
             impl Rundo for #name {

                type Op = #op_name;

                fn dirty(&self) -> bool {
                    #dirty_method
                }

                fn reset(&mut self) {
                    #reset_impl
                }

                fn change_op(&self)-> Option<#op_name> {
                    match self.dirty() {
                        true => {Some( #op_name { #ops_impl })},
                        false => None
                    }
                }

                fn back(&mut self, op: &Self::Op) {
                    #back_impl;
                }

                fn forward(&mut self, op: &Self::Op) {
                    #forward_impl
                }
            }
        }
    }
}

pub trait RundoFields {
    fn named_filed_only(&self) -> &Punctuated<Field, Comma>;
    fn filter_rundo_skip(&self) -> Vec<&Field>;
    fn fields_def(&self) -> quote::Tokens;
    fn op_def(&self) -> quote::Tokens;
    fn op_method(&self) -> quote::Tokens;
    fn reset_method(&self) -> quote::Tokens;
    fn back_method(&self) -> quote::Tokens;
    fn forward_method(&self) -> quote::Tokens;
    fn dirty_method(&self) -> quote::Tokens;
}

fn rundo_field_metas(field: &Field) -> Vec<Vec<syn::NestedMeta>> {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "rundo" {
                if let Some(List(ref meta)) = attr.interpret_meta() {
                    return Some(meta.nested.iter().cloned().collect::<Vec<_>>());
                }
            }
            None
        })
        .collect::<Vec<_>>()
}

fn is_skip_field(field: &Field) -> bool {
    rundo_field_metas(field).iter().any(|metas| {
        metas.iter().any(|meta| match meta {
            &Meta(Word(ref word)) => word == "skip",
            _ => false,
        })
    })
}

pub fn is_inner_rundo_type(field: &Field) -> bool {
    if is_skip_field(field) {
        return false;
    }

    if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
        path.segments.last().map_or(false, |pair| {
            let tt_id = pair.value().ident.as_ref();
            IMPLED_RUNDO.iter().any(|t| t == &tt_id)
        })
    } else {
        false
    }
}

fn rundo_type_def(field: &Field) -> quote::Tokens {
    let ty = &field.ty;
    if is_inner_rundo_type(field) {
        quote!{ValueType<#ty>}
    } else {
        quote!{#ty}
    }
}

impl RundoFields for Fields {
    fn named_filed_only(&self) -> &Punctuated<Field, Comma> {
        match self {
            &Fields::Named(ref fs) => &fs.named,
            &Fields::Unnamed(_) => panic!("rundo not support tuple struct now"),
            &Fields::Unit => panic!("rundo not support unit struct now"),
        }
    }

    fn filter_rundo_skip(&self) -> Vec<&Field> {
        self.named_filed_only()
            .iter()
            .filter(|fd| !is_skip_field(fd))
            .collect::<Vec<_>>()
    }

    fn fields_def(&self) -> quote::Tokens {
        let defs = self.named_filed_only()
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref();
                let ty = rundo_type_def(&field);
                let vis = &field.vis;
                quote!{ #vis #ident: #ty, }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }

    fn op_def(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref();
                let ty = rundo_type_def(&field);
                let op = quote! { <#ty as Rundo>::Op };
                let ops_type = quote!{ Option<#op>};
                quote!{ #ident: #ops_type, }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }

    fn op_method(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! { #ident: self.#ident.change_op(), }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }

    fn dirty_method(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! { self.#ident.dirty() }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs) ||* }
    }

    fn reset_method(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! { self.#ident.reset(); }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }

    fn back_method(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! {
                    if let Some(ref op) = op.#ident {
                        self.#ident.back(&op);
                    }
                    self.reset();
                }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }

    fn forward_method(&self) -> quote::Tokens {
        let defs = self.filter_rundo_skip()
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! {
                    if let Some(ref op) = op.#ident {
                        self.#ident.forward(&op);
                    }
                    self.reset();
                }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs)* }
    }
}
