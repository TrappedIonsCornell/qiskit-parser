use crate::instruction::{Instruction, InstructionType};
use crate::bit::{Qubit, Clbit};

#[derive(Debug, PartialEq, Clone)]
pub struct CircuitInstruction {
    operation: InstructionType,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl CircuitInstruction {
    pub fn new(operation: Instruction, qubits: Vec<Qubit>, clbits: Vec<Clbit>) -> Self {
        CircuitInstruction {
            operation: InstructionType::from(operation),
            qubits,
            clbits,
        }
    }

    pub fn get_operation(&self) -> &InstructionType {
        &self.operation
    }

    pub fn get_qubits(&self) -> &Vec<Qubit> {
        &self.qubits
    }

    pub fn get_clbits(&self) -> &Vec<Clbit> {
        &self.clbits
    }
}