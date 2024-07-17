use crate::instruction::Instruction;
use crate::bit::{Qubit, Clbit};

#[derive(Debug, PartialEq, Clone)]
pub struct CircuitInstruction {
    operation: Instruction,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl CircuitInstruction {
    pub fn new(operation: Instruction, qubits: Vec<Qubit>, clbits: Vec<Clbit>) -> Self {
        CircuitInstruction {
            operation,
            qubits,
            clbits,
        }
    }

    pub fn get_operation(&self) -> &Instruction {
        &self.operation
    }

    pub fn get_qubits(&self) -> &Vec<Qubit> {
        &self.qubits
    }

    pub fn get_clbits(&self) -> &Vec<Clbit> {
        &self.clbits
    }
}