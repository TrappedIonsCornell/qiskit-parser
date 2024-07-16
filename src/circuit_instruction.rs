use crate::instruction::Instruction;
use crate::bit::Qubit;
use crate::bit::Clbit;

pub struct CircuitInstruction {
    operation: Instruction,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}