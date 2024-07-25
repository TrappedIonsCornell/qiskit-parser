extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn call_all_functions(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);

    let mod_name = &input.ident;
    let mut call_statements = Vec::new();

    if let Some((_, ref items)) = input.content {
        for item in items.iter() {
            if let syn::Item::Fn(f) = item {
                let fn_name = &f.sig.ident;
                call_statements.push(quote! {
                    #mod_name::#fn_name();
                });
            }
        }
    }

    let calls = quote! {
        fn call_all() {
            #(#call_statements)*
        }
    };

    let code = quote! {
        #input

        #calls
    };

    TokenStream::from(code)
}