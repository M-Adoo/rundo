use quote;
use syn;

use literal;

pub fn prefix_ident(ident: &syn::Ident, prefix: &str) -> syn::Ident {
    let in_name = prefix.to_owned() + ident.as_ref();
    syn::Ident::from(in_name)
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

pub trait RundoStruct {
    fn inner_name(&self) -> syn::Ident;
    fn op_name(&self) -> syn::Ident;
    fn struct_def(&self) -> quote::Tokens;
    fn inner_struct_def(&self) -> quote::Tokens;
    fn op_struct_def(&self) -> quote::Tokens;
    fn impl_rundo(&self) -> quote::Tokens;
    fn impl_ref_deref(&self) -> quote::Tokens;
}

impl RundoStruct for syn::ItemStruct {
    fn inner_name(&self) -> syn::Ident {
        prefix_ident(&self.ident, "_")
    }

    fn op_name(&self) -> syn::Ident {
        prefix_ident(&self.ident, "Op")
    }

    fn struct_def(&self) -> quote::Tokens {
        let vis = &self.vis;
        let name = &self.ident;
        let inner_name = self.inner_name();
        quote! {
                #vis struct #name {
                value: #inner_name,
                dirty: bool,
            }
        }
    }

    fn inner_struct_def(&self) -> quote::Tokens {
        let vis = &self.vis;
        let name = self.inner_name();
        let fields_def = self.fields.fields_def();
        quote!{
            #vis struct #name { #fields_def }
        }
    }

    fn op_struct_def(&self) -> quote::Tokens {
        let vis = &self.vis;
        let name = self.op_name();
        let ops_def = self.fields.op_def();
        quote! { #vis struct #name { #ops_def } }
    }

    fn impl_rundo(&self) -> quote::Tokens {
        let name = &self.ident;
        let op_name = &self.op_name();
        let reset_impl = self.fields.reset_method();
        let ops_impl = self.fields.op_method();
        let back_impl = self.fields.back_method();
        let forward_impl = self.fields.forward_method();
        quote! {
             impl Rundo for #name {

                type Op = #op_name;

                fn dirty(&self) -> bool{
                    self.dirty
                }

                fn reset(&mut self) {
                    self.dirty = false;
                    #reset_impl
                }

                fn change_op(&self)-> Option<#op_name> {
                    match self.dirty {
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

    fn impl_ref_deref(&self) -> quote::Tokens {
        let name = &self.ident;
        let inner_name = self.inner_name();
        quote!{
            impl std::ops::Deref for #name {
                type Target = #inner_name;
                fn deref(&self) -> &#inner_name { &self.value }
            }

            impl std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut #inner_name {
                        if !self.dirty {
                        self.dirty = true;
                    }
                    &mut self.value
                }
            }
        }
    }
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
}

impl RundoFields for syn::Fields {
    fn fields_def(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            let vis = &field.vis;
            quote!{ #vis #ident: ValueType<#ty>, }
        })
    }

    fn op_def(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = field.ident.as_ref();
            let ty = &field.ty;
            let op = quote! { <ValueType<#ty> as Rundo>::Op };
            let ops_type = quote!{ Option<#op>};
            quote!{ #ident: #ops_type, }
        })
    }

    fn op_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { #ident: self.value.#ident.change_op(), }
        })
    }

    fn reset_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { self.value.#ident.reset();}
        })
    }

    fn back_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { 
                if let Some(ref op) = op.#ident {
                    self.value.#ident.back(&op);
                }
                self.dirty = false;
            }
        })   
    }

     fn forward_method(&self) -> quote::Tokens {
        fields_map(self, |field| {
            let ident = &field.ident;
            quote! { 
                if let Some(ref op) = op.#ident {
                    self.value.#ident.forward(&op); 
                }
                self.dirty = false;
            }
        })   
    }
}
