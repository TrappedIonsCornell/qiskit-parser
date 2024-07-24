extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Operation)]
/// Derive macro generating an impl of the trait `Operation`
pub fn derive_operation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let gen = quote! {
        impl Operation for #name {
            fn params(&self) -> &Vec<f64> {
                self.instruction.params()
            }

            fn duration(&self) -> Option<f64> {
                self.instruction.duration()
            }

            fn unit(&self) -> Unit {
                self.instruction.unit()
            }

            fn set_params(&mut self, params: Vec<f64>) {
                self.instruction.set_params(params);
            }

            fn set_duration(&mut self, duration: Option<f64>) {
                self.instruction.set_duration(duration);
            }

            fn set_unit(&mut self, unit: Unit) {
                self.instruction.set_unit(unit);
            }

        }
    };
    gen.into()
}