use quote;
use syn;
use rundo_types::IMPLED_RUNDO;

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

// fields
macro_rules! named_filed_only {
    ($ident:expr) => {
         match $ident {
            &syn::Fields::Named(ref fs) => fs,
            &syn::Fields::Unnamed(_) => panic!("rundo not support tuple struct now"),
            &syn::Fields::Unit =>panic!("rundo not support unit struct now"),
        };
    };
}

pub fn fields_map<F>(fields: &syn::Fields, field_to_token: F) -> quote::Tokens
where
    F: FnMut(&syn::Field) -> quote::Tokens,
{
    let defs = named_filed_only!(fields)
        .named
        .iter()
        .map(field_to_token)
        .collect::<Vec<_>>();
    quote!{ #(#defs)* }
}

trait RundoFields {
    fn fields_def(&self) -> quote::Tokens;
    fn op_def(&self) -> quote::Tokens;
    fn op_method(&self) -> quote::Tokens;
    fn reset_method(&self) -> quote::Tokens;
    fn back_method(&self) -> quote::Tokens;
    fn forward_method(&self) -> quote::Tokens;
    fn dirty_method(&self) -> quote::Tokens;
}

pub fn default_ty_impled(ty: &syn::Type) -> bool {
    if let &syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        path.segments.last().map_or(false, |pair| {
            let tt_id = pair.value().ident.as_ref();
            IMPLED_RUNDO.iter().any(|t| t == &tt_id)
        })
    } else {
        false
    }
}

fn rundo_type_def(ty: &syn::Type) -> quote::Tokens {
    if default_ty_impled(ty) {
        quote!{ValueType<#ty>}
    } else {
        quote!{#ty}
    }
}

impl RundoFields for syn::Fields {
    fn fields_def(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = field.ident.as_ref();
            let ty = rundo_type_def(&field.ty);
            let vis = &field.vis;
            quote!{ #vis #ident: #ty, }
        })
    }

    fn op_def(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = field.ident.as_ref();
            let ty = rundo_type_def(&field.ty);
            let op = quote! { <#ty as Rundo>::Op };
            let ops_type = quote!{ Option<#op>};
            quote!{ #ident: #ops_type, }
        })
    }

    fn op_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { #ident: self.#ident.change_op(), }
        })
    }

    fn dirty_method(&self) -> quote::Tokens {
        let defs = named_filed_only!(self)
            .named
            .iter()
            .map(|field| {
                let ident = &field.ident;
                quote! { self.#ident.dirty() }
            })
            .collect::<Vec<_>>();
        quote!{ #(#defs) ||* }
    }

    fn reset_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { self.#ident.reset(); }
        })
    }

    fn back_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! {
                if let Some(ref op) = op.#ident {
                    self.#ident.back(&op);
                }
                self.reset();
            }
        })
    }

    fn forward_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! {
                if let Some(ref op) = op.#ident {
                    self.#ident.forward(&op);
                }
                self.reset();
            }
        })
    }
}
