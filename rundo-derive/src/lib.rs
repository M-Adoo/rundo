extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;


use proc_macro::TokenStream;

#[proc_macro_derive(Rundo)]
pub fn rundo(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_rundo_derive(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_rundo_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Rundo for #name {
            fn diff() -> i32 {
               4
            }
        }
    }
}
