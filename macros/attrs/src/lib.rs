#![recursion_limit = "128"]
#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate rundo_types;
extern crate syn;

mod literal;
mod rundo_struct;

use literal::LiteralMacro;
use rundo_struct::RundoStruct;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rundo(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Well, can't parse the code, maybe some syntax error!");
    impl_rundo_attrs(&ast).into()
}

fn impl_rundo_attrs(item: &syn::Item) -> quote::Tokens {
    if let &syn::Item::Struct(ref s) = item {
        let op_def = s.op_struct_def();
        let struct_def = s.struct_def();
        let impl_rundo = s.impl_rundo();
        let literal_macro = s.literal_macro();

        quote! {
            #op_def

            #struct_def

            #impl_rundo

            #literal_macro
        }
    } else {
        panic!("#[rundo] is only support for structs now!");
    }
}
