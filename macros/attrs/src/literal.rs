#![recursion_limit = "128"]

#[macro_use]
use quote;
use syn;

use rundo_struct::*;

pub trait LiteralMacro {
    fn literal_macro(&self) -> quote::Tokens;
}

impl LiteralMacro for syn::ItemStruct {
    fn literal_macro(&self) -> quote::Tokens {
        let field_match = self.fields.named_filed_only().iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            let init_field = |init_expr| {
                if is_inner_rundo_type(field) {
                    quote!{ValueType::<#ty>::from(#init_expr)}
                } else {
                    quote!{#init_expr}
                }
            };
            let shorthand = init_field(quote!{#ident});
            let normal = init_field(quote!{$e});
            quote!{
                // match shorthand literal field constuct like
                // {a, b, c}
                (#ident) => ( #shorthand);
                // normal struct literal field construtc like
                // {a: 1, b: 1, c: 1}
                (#ident : $e:expr) => ( #normal);
            }
        });

        let name = self.ident;
        let field_macro = prefix_ident(&self.ident, "_field_");
        let construct = |field_exp| {
            quote! {
                #name {
                        $($id: #field_macro!(#field_exp)),*
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
                ($($id: ident : $e: expr ,) *) => { #normal };
                ($($id: ident : $e: expr ), *) => { #normal };
            }
        }
    }
}
