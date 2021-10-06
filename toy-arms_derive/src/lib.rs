extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GameObject)]
pub fn game_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_game_object(&ast)
}

fn impl_game_object(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GameObject for #name {
            unsafe fn from_raw(address: *const usize) -> Option<*mut #name> {
                let ptr = address as *mut Self;
                if (ptr as *mut usize).is_null() {
                    return None
                }
                Some(ptr)
            }
        }
    };
    gen.into()
}