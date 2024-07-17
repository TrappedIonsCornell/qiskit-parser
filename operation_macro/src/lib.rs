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
            fn name(&self) -> &str {
                self.instruction.name()
            }

            fn num_qubits(&self) -> usize {
                self.instruction.num_qubits()
            }

            fn num_clbits(&self) -> usize {
                self.instruction.num_clbits()
            }

            fn params(&self) -> &[f64] {
                self.instruction.params()
            }

            fn duration(&self) -> Option<f64> {
                self.instruction.duration()
            }

            fn unit(&self) -> Option<&str> {
                self.instruction.unit()
            }

            fn label(&self) -> Option<&str> {
                self.instruction.label()
            }
        }
    };
    gen.into()
}