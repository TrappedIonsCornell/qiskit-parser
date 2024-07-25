use crate::operations::Operation;

/// Description of a Qiskit Circuit element. Provides a specific operation and the
/// qubits/classical bits it interacts with.
#[derive(Debug, PartialEq, Clone)]
pub struct CircuitInstruction {
    operation: Operation,
    qubits: Vec<usize>,
    clbits: Vec<usize>,
}

impl CircuitInstruction {
    /// Create a new CircuitInstruction.
    pub fn new(operation: Operation, qubits: Vec<usize>, clbits: Vec<usize>) -> Self {
        Self {
            operation,
            qubits,
            clbits,
        }
    }

    /// Get the operation of the CircuitInstruction.
    pub fn get_operation(&self) -> &Operation {
        &self.operation
    }

    /// Get the qubits of the CircuitInstruction.
    pub fn get_qubits(&self) -> &Vec<usize> {
        &self.qubits
    }

    /// Get the classical bits of the CircuitInstruction.
    pub fn get_clbits(&self) -> &Vec<usize> {
        &self.clbits
    }
}