use crate::{instruction::Operation, util::pool::Handle};

/// Description of a Qiskit Circuit element. Provides a specific operation and the
/// qubits/classical bits it interacts with.
#[derive(Clone)]
pub struct CircuitInstruction {
    operation: Handle<Box<dyn Operation>>,
    qubits: Vec<usize>,
    clbits: Vec<usize>,
}

impl CircuitInstruction {
    pub fn new(operation: Handle<Box<dyn Operation>>, qubits: Vec<usize>, clbits: Vec<usize>) -> Self {
        CircuitInstruction {
            operation,
            qubits,
            clbits,
        }
    }

    pub fn get_operation(&self) -> &Handle<Box<dyn Operation>> {
        &self.operation
    }

    pub fn get_qubits(&self) -> &Vec<usize> {
        &self.qubits
    }

    pub fn get_clbits(&self) -> &Vec<usize> {
        &self.clbits
    }
}