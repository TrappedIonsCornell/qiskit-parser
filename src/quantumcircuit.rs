use crate::{bit::{Clbit, Qubit}, circuit_instruction::CircuitInstruction, instruction::Instruction, parser::Parser, util::pool::{AsPool, Pool}};

pub struct QuantumCircuit {
    instr: Vec<CircuitInstruction>,
    gates: Pool<Instruction, ()>,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl QuantumCircuit {
    pub fn new(circuit_data: String) -> Self {
        QuantumCircuit {
            instr: Parser::new(circuit_data).parse(),
            gates: Pool::new().expect("Failed to create pool"),
            qubits: Vec::new(),
            clbits: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, gate: Instruction){
        self.gates.add(gate);
    }

    pub fn get_instructions(&self) -> &Vec<CircuitInstruction> {
        &self.instr
    }

    pub fn get_qubits(&self) -> &Vec<Qubit> {
        &self.qubits
    }

    pub fn get_clbits(&self) -> &Vec<Clbit> {
        &self.clbits
    }
}
