extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro]
pub fn generate_insert_gates(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);
    let mod_name = &input.ident;

    // Extract function names from the module
    let mut gate_names = Vec::new();

    if let Some((_, items)) = input.content {
        for item in items {
            if let syn::Item::Fn(func) = item {
                let func_name = func.sig.ident.to_string();
                gate_names.push(func_name);
            }
        }
    }

    // Generate the map insertion code
    let insertions = gate_names.iter().map(|name| {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            mtx_map.insert(
                singleton::#ident().name().to_string(),
                singleton::#ident().to_matrix(),
            );
        }
    });

    let expanded = quote! {
        {
            let mut mtx_map = std::collections::HashMap::new();
            #(#insertions)*
            mtx_map
        }
    };

    TokenStream::from(expanded)
}