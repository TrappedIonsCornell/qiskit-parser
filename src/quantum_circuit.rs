mod parser;
mod tokenizer;

use crate::{
    bit::{Clbit, Qubit},
    circuit_instruction::CircuitInstruction,
    operations::{Gate, Operation},
};

pub struct QuantumCircuit {
    instr: Vec<CircuitInstruction>,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl QuantumCircuit {
    pub fn new(input: String, custom_gates: Option<Vec<Gate>>) -> Self {
        let mut operations: Vec<Operation> = Vec::new();
        if let Some(c_gates) = custom_gates {
            operations.extend(c_gates.into_iter().map(|gate| Operation::Gate(gate)));
        }

        let mut parser = parser::Parser::new(input);
        let mut qubits: Vec<Qubit> = vec![];
        let mut clbits: Vec<Clbit> = vec![];

        let instr: Vec<CircuitInstruction> =
            parser.parse(&mut operations, &mut qubits, &mut clbits);

        QuantumCircuit {
            instr,
            qubits,
            clbits,
        }
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
    use crate::gates::singleton as singleton_gates;

    use super::*;

    /// Testing an X gate /
    #[test]
    fn test_single_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let qc = QuantumCircuit::new(input.to_string(), None);

        let instructions = qc.get_instructions();
        assert_eq!(instructions.len(), 1);

        let instr = instructions.get(0).unwrap();
        assert_eq!(instr, &CircuitInstruction::new(
            Operation::Gate(singleton_gates::x()),
            vec![0],
            vec![],
        ));
    }

    /// Testing an X and Y gate
    #[test]
    fn test_single_qubit_two_gates() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='y', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let qc = QuantumCircuit::new(input.to_string(), None);

        let instructions = qc.get_instructions();
        assert_eq!(instructions.len(), 2);

        let instr = instructions.get(0).unwrap();
        assert_eq!(instr, &CircuitInstruction::new(
            Operation::Gate(singleton_gates::x()),
            vec![0],
            vec![],
        ));

        let instr = instructions.get(1).unwrap();
        assert_eq!(instr, &CircuitInstruction::new(
            Operation::Gate(singleton_gates::y()),
            vec![0],
            vec![],
        ));
    }

    // /// Testing a CNOT gate
    // #[test]
    // fn test_two_qubit_one_gate() {
    //     let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

    //     let qc = QuantumCircuit::new(input.to_string(), None);

    //     let instructions = qc.get_instructions();
    //     assert_eq!(instructions.len(), 1);

    //     let instr = instructions.get(0).unwrap();
    //     assert_eq!(instr, &CircuitInstruction::new(
    //         Operation::Gate(singleton_gates::cx()),
    //         vec![0, 1],
    //         vec![],
    //     ));

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
