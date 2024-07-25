use crate::operations::Operation;

/// Description of a Qiskit Circuit element. Provides a specific operation and the
/// qubits/classical bits it interacts with.
#[derive(Clone)]
pub struct CircuitInstruction<'a> {
    operation: &'a Operation,
    qubits: Vec<usize>,
    clbits: Vec<usize>,
}

impl CircuitInstruction<'_> {
    /// Create a new `CircuitInstruction` with the given operation, qubits and classical bits.
    pub fn new(operation: &Operation, qubits: Vec<usize>, clbits: Vec<usize>) -> Self {
        CircuitInstruction {
            operation,
            qubits,
            clbits,
        }
    }

    /// Get the operation of the `CircuitInstruction`.
    pub fn operation(&self) -> &Operation {
        self.operation
    }

    /// Get the qubits of the `CircuitInstruction`.
    pub fn qubits(&self) -> &Vec<usize> {
        &self.qubits
    }

    /// Get the classical bits of the `CircuitInstruction`.
    pub fn clbits(&self) -> &Vec<usize> {
        &self.clbits
    }
}
