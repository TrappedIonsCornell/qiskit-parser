use std::{collections::HashMap, io};
mod parser;
mod tokenizer;

use crate::{
    bit::{Clbit, Qubit},
    circuit_instruction::CircuitInstruction,
    instruction::{Operation, OperationPool},
    util::pool::{Handle, ARENA_SIZE_BYTES},
};

pub struct QuantumCircuit {
    instr: Vec<CircuitInstruction>,
    gates: OperationPool,
    gate_lookup: HashMap<String, Handle<Box<dyn Operation>>>,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl QuantumCircuit {
    pub fn new(input: String) -> io::Result<Self> {
        let mut parser = parser::Parser::new(input);
        let mut gates = OperationPool::new(ARENA_SIZE_BYTES)?;
        let mut qubits: Vec<Qubit> = vec![];
        let mut clbits: Vec<Clbit> = vec![];
        let mut gate_lookup = HashMap::new();

        let instr: Vec<CircuitInstruction> =
            parser.parse(&mut gates, &mut gate_lookup, &mut qubits, &mut clbits);

        Ok(QuantumCircuit {
            instr,
            gates,
            gate_lookup,
            qubits,
            clbits,
        })
    }

    pub fn add_gate(&mut self, gate_alias: String, gate_struct: &dyn Operation) {
        let handle = self.gates.add(gate_struct);
        self.gate_lookup.insert(gate_alias, handle);
    }

    pub fn get_gates(&self) -> &OperationPool {
        &self.gates
    }

    pub fn get_gate_lookup(&self) -> &HashMap<String, Handle<Box<dyn Operation>>> {
        &self.gate_lookup
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

    pub fn add_qubit(&mut self, qubit: Qubit) {
        if self.qubits.contains(&qubit) {
            return;
        }
        self.qubits.push(qubit);
    }

    pub fn add_clbit(&mut self, clbit: Clbit) {
        if self.clbits.contains(&clbit) {
            return;
        }
        self.clbits.push(clbit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Testing an X gate
    #[test]
    fn test_single_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let qc = QuantumCircuit::new(input.to_string()).unwrap();

        let instructions = qc.get_instructions();
        assert_eq!(instructions.len(), 1);

        let instr = instructions.get(0).unwrap();
        let gate = qc.get_gates().get(*instr.get_operation());
        // println!("{:?}", gate.params());
    }

    // /// Testing an X and Y gate
    // #[test]
    // fn test_single_qubit_two_gates() {
    //     let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='y', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

    //     let parser = QuantumCircuit::new(input.to_string());
    // }

    // /// Testing a CNOT gate
    // #[test]
    // fn test_two_qubit_one_gate() {
    //     let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

    //     let parser = QuantumCircuit::new(input.to_string());
    // }

    // /// Testing the Bell state circuit
    // #[test]
    // fn test_bell_state() {
    //     let input = "[CircuitInstruction(operation=Instruction(name='h', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

    //     let parser = QuantumCircuit::new(input.to_string());
    // }

    // /// Testing the classic |0> -> |000> QECC circuit
    // #[test]
    // fn test_naive_qecc() {
    //     let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(3, 'q'), 0), Qubit(QuantumRegister(3, 'q'), 1)), clbits=()), CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(3, 'q'), 0), Qubit(QuantumRegister(3, 'q'), 2)), clbits=())]";

    //     let parser = QuantumCircuit::new(input.to_string());
    // }
}
