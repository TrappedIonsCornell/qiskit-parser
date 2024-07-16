use crate::register::ClassicalRegister;
use crate::register::QuantumRegister;
use crate::register::Register;
use crate::register::RegisterOps;

use crate::bit::Bit;
use crate::bit::BitOps;
use crate::bit::Clbit;
use crate::bit::Qubit;

use crate::circuit_instruction::CircuitInstruction;
use crate::instruction::Instruction;

pub struct Parser {
    input: String,
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser { input }
    }

    pub fn parse(&self, s: String) -> Vec<CircuitInstruction> {
        let mut instructions = Vec::new();

        // Remove the surrounding brackets
        let s = s.trim_start_matches('[').trim_end_matches(']');

        // Split by CircuitInstruction
        for circuit_instr in s.split("CircuitInstruction(").skip(1) {
            let circuit_instr = circuit_instr.trim_end_matches(')').trim_end_matches(',');

            // Parse operation
            let operation_start = circuit_instr.find("operation=Instruction(").unwrap()
                + "operation=Instruction(".len();
            let operation_end = circuit_instr.find("), qubits=").unwrap();
            let operation_str = &circuit_instr[operation_start..operation_end];

            let operation = self.parse_instruction(operation_str.to_string());

            // Parse qubits
            let qubits_start = circuit_instr.find("qubits=(").unwrap() + "qubits=(".len();
            let qubits_end = circuit_instr.find("), clbits=").unwrap();
            let qubits_str = &circuit_instr[qubits_start..qubits_end];
            let qubits = self.parse_qubits(qubits_str.to_string());

            // Parse clbits
            let clbits_start = circuit_instr.find("clbits=(").unwrap() + "clbits=(".len();
            let clbits_end = circuit_instr.len();
            let clbits_str = &circuit_instr[clbits_start..clbits_end];
            let clbits = self.parse_clbits(clbits_str.to_string());

            instructions.push(CircuitInstruction::new(operation, qubits, clbits));
        }

        instructions
    }

    fn parse_instruction(&self, s: String) -> Instruction {
        let name = self.extract_value(&s, "name='", "'");
        let num_qubits = self.extract_value(&s, "num_qubits=", ", ").parse().unwrap();
        let num_clbits = self.extract_value(&s, "num_clbits=", ", ").parse().unwrap();
        let params_str = self.extract_value(&s, "params=[", "]");
        let params = params_str
            .split(",")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let duration = self
            .extract_optional_value(&s, "duration=", ", ")
            .and_then(|v| v.parse().ok());
        let unit = self.extract_optional_value(&s, "unit='", "'");
        let label = self.extract_optional_value(&s, "label='", "'");

        Instruction::new(name, num_qubits, num_clbits, params, duration, unit, label)
    }

    fn parse_qubits(&self, s: String) -> Vec<Qubit> {
        let mut qubits = Vec::new();

        for qubit_str in s.split("Qubit(").skip(1) {
            let qubit_str = qubit_str.trim_end_matches(')').trim_end_matches(',');
            qubits.push(self.parse_qubit(qubit_str.to_string()));
        }

        qubits
    }

    fn parse_clbits(&self, s: String) -> Vec<Clbit> {
        let mut clbits = Vec::new();

        for clbit_str in s.split("Clbit(").skip(1) {
            let clbit_str = clbit_str.trim_end_matches(')').trim_end_matches(',');
            clbits.push(self.parse_clbit(clbit_str.to_string()));
        }

        clbits
    }

    fn parse_qubit(&self, s: String) -> Qubit {
        let register_start = s.find("QuantumRegister(").unwrap() + "QuantumRegister(".len();
        let register_end = s.find("), ").unwrap();
        let register_str = &s[register_start..register_end];

        let register = Box::new(Register::QuantumRegister(
            self.parse_quantum_register(register_str.to_string()),
        ));

        let index = self.extract_value(&s[register_end..], ", ", ")");
        let a = index.parse().unwrap();

        Qubit::new(*register, a)
    }

    fn parse_clbit(&self, s: String) -> Clbit {
        let register_start = s.find("ClassicalRegister(").unwrap() + "ClassicalRegister(".len();
        let register_end = s.find("), ").unwrap();
        let register_str = &s[register_start..register_end];

        let register = Box::new(Register::ClassicalRegister(
            self.parse_classical_register(register_str.to_string()),
        ));
        let index = self
            .extract_value(&s[register_end..], ", ", ")")
            .parse()
            .unwrap();

        Clbit::new(*register, index)
    }

    fn parse_quantum_register(&self, s: String) -> QuantumRegister {
        let size = self
            .extract_optional_value(&s, "", ", ")
            .and_then(|v| v.parse().ok());
        let name = self.extract_optional_value(&s, "'", "'");
        let bits_str = self
            .extract_optional_value(&s, "bits=", "")
            .unwrap_or_default();
        let bits = if bits_str.is_empty() {
            None
        } else {
            Some(self.parse_bit(bits_str))
        };

        QuantumRegister::new(size, name, bits)
    }

    fn parse_classical_register(&self, s: String) -> ClassicalRegister {
        let size = self
            .extract_optional_value(&s, "", ", ")
            .and_then(|v| v.parse().ok());
        let name = self.extract_optional_value(&s, "'", "'");
        let bits_str = self
            .extract_optional_value(&s, "bits=", "")
            .unwrap_or_default();
        let bits = if bits_str.is_empty() {
            None
        } else {
            Some(self.parse_bit(bits_str))
        };

        ClassicalRegister::new(size, name, bits)
    }

    fn parse_bit(&self, s: String) -> Bit {
        if s.starts_with("Qubit(") {
            Bit::Qubit(
                self.parse_qubit(
                    s.trim_start_matches("Qubit(")
                        .trim_end_matches(")")
                        .to_string(),
                ),
            )
        } else if s.starts_with("Clbit(") {
            Bit::Clbit(
                self.parse_clbit(
                    s.trim_start_matches("Clbit(")
                        .trim_end_matches(")")
                        .to_string(),
                ),
            )
        } else {
            panic!("Unknown bit type")
        }
    }

    fn extract_value(&self, s: &str, start: &str, end: &str) -> String {
        let start_idx = s.find(start).unwrap() + start.len();
        let end_idx = s[start_idx..].find(end).unwrap() + start_idx;
        s[start_idx..end_idx].to_string()
    }

    fn extract_optional_value(&self, s: &str, start: &str, end: &str) -> Option<String> {
        if let Some(start_idx) = s.find(start) {
            let start_idx = start_idx + start.len();
            if let Some(end_idx) = s[start_idx..].find(end) {
                let end_idx = end_idx + start_idx;
                Some(s[start_idx..end_idx].to_string())
            } else {
                Some(s[start_idx..].to_string())
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let parser = Parser::new(input.to_string());
        let instructions = parser.parse(input.to_string());

        assert_eq!(instructions.len(), 1);

        let mut instr = &instructions[0];
        assert_eq!(
            instr,
            &CircuitInstruction::new(
                Instruction::new("x".to_string(), 1, 0, vec![], None, None, None),
                vec![Qubit::new(
                    Register::QuantumRegister(QuantumRegister::new(
                        Some(1),
                        Some("q".to_string()),
                        None
                    )),
                    0,
                )],
                vec![],
            )
        );

    }

    #[test]
    fn test_single_qubit_two_gates() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='y', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let parser = Parser::new(input.to_string());
        let instructions = parser.parse(input.to_string());

        assert_eq!(instructions.len(), 2);

        let mut instr = &instructions[0];
        assert_eq!(
            instr,
            &CircuitInstruction::new(
                Instruction::new("x".to_string(), 1, 0, vec![], None, None, None),
                vec![Qubit::new(
                    Register::QuantumRegister(QuantumRegister::new(
                        Some(1),
                        Some("q".to_string()),
                        None
                    )),
                    0,
                )],
                vec![],
            )
        );

        instr = &instructions[1];
        assert_eq!(
            instr,
            &CircuitInstruction::new(
                Instruction::new("y".to_string(), 1, 0, vec![], None, None, None),
                vec![Qubit::new(
                    Register::QuantumRegister(QuantumRegister::new(
                        Some(1),
                        Some("q".to_string()),
                        None
                    )),
                    0,
                )],
                vec![],
            )
        );
    }

    #[test]
    fn test_two_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

        let parser = Parser::new(input.to_string());
        let instructions = parser.parse(input.to_string());

        assert_eq!(instructions.len(), 1);
        let instr = &instructions[0];
        assert_eq!(
            instr,
            &CircuitInstruction::new(
                Instruction::new("cx".to_string(), 2, 0, vec![], None, None, None),
                vec![
                    Qubit::new(
                        Register::QuantumRegister(QuantumRegister::new(
                            Some(2),
                            Some("q".to_string()),
                            None
                        )),
                        0,
                    ),
                    Qubit::new(
                        Register::QuantumRegister(QuantumRegister::new(
                            Some(2),
                            Some("q".to_string()),
                            None
                        )),
                        1,
                    )
                ],
                vec![],
            )
        );
    }
}
