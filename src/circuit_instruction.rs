use crate::bit::{Qubit, Clbit};

/// Description of a Qiskit Circuit element. Provides a specific operation and the
/// qubits/classical bits it interacts with.
#[derive(Debug, PartialEq, Clone)]
pub struct CircuitInstruction {
    operation: InstructionType,
    qubits: Vec<i32>,
    clbits: Vec<i32>,
}

impl CircuitInstruction {
    pub fn new(operation: Instruction, qubits: Vec<i32>, clbits: Vec<i32>) -> Self {
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