use nalgebra::Complex;

pub mod circuit_instruction;
pub mod operations;
pub mod bit;
pub mod gates;
pub mod quantum_circuit;

#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;