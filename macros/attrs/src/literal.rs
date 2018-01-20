#![recursion_limit = "128"]

#[macro_use]
use quote;
use syn;

use rundo_struct::RundoStruct;
use rundo_struct;

pub trait LiteralMacro {
    fn literal_macro(&self) -> quote::Tokens;
}

impl LiteralMacro for syn::ItemStruct {
    fn literal_macro(&self) -> quote::Tokens {
        let inner_type = self.inner_name();
        let field_match = rundo_struct::fields_map(&self.fields, |field| {
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

        let name = self.ident;
        let field_macro = rundo_struct::prefix_ident(&self.ident, "_field_");
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
}
