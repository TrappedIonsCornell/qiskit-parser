use ndarray::Array2;
use numpy::Complex64;

use crate::instruction::{Gate, Instruction, Operation};
use operation_macro::Operation;

#[derive(Debug, PartialEq, Clone, Operation)]
/// A Pauli-X gate
pub struct XGate {
    instruction: Instruction,
}

impl XGate {
    pub fn new() -> Self {
        XGate {
            instruction: Instruction::new(String::from("x"), 1, 0, vec![], None, None, None),
        }
    }
}

impl Gate for XGate {
    fn to_matrix(&self) -> Array2<Complex64> {
        Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Operation)]
/// A Pauli-Y gate
struct YGate {
    instruction: Instruction,
}

impl YGate {
    pub fn new() -> Self {
        YGate {
            instruction: Instruction::new(String::from("y"), 1, 0, vec![], None, None, None),
        }
    }
}

impl Gate for YGate {
    fn to_matrix(&self) -> Array2<Complex64> {
        Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(0.0, 1.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Operation)]
/// A Pauli-Z gate
struct ZGate {
    instruction: Instruction,
}

impl ZGate {
    pub fn new() -> Self {
        ZGate {
            instruction: Instruction::new(String::from("z"), 1, 0, vec![], None, None, None),
        }
    }
}

impl Gate for ZGate {
    fn to_matrix(&self) -> Array2<Complex64> {
        Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0),
            ],
        )
        .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Operation)]
/// A Hadamard gate
pub struct HadamardGate {
    instruction: Instruction,
}

impl HadamardGate {
    pub fn new() -> Self {
        HadamardGate {
            instruction: Instruction::new(String::from("h"), 1, 0, vec![], None, None, None),
        }
    }
}

impl Gate for HadamardGate {
    fn to_matrix(&self) -> Array2<Complex64> {
        let h = 1.0 / 2.0_f64.sqrt();
        Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(h, 0.0),
                Complex64::new(h, 0.0),
                Complex64::new(h, 0.0),
                Complex64::new(-h, 0.0),
            ],
        )
        .unwrap()
    }
}
