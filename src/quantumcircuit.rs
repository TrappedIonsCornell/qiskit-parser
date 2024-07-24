use std::{collections::HashMap, io};

// use list_any::{VecAny, VecAnyGuard};

use crate::{
    bit::{Clbit, Qubit},
    circuit_instruction::CircuitInstruction,
    gates::singleton::{HadamardGate, XGate, YGate, ZGate},
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

mod tokenizer {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        OpenBracket,
        CloseBracket,
        OpenParen,
        CloseParen,
        Comma,
        Equals,
        Identifier(String),
        StringLiteral(String),
        Number(f64),
    }
    pub struct Tokenizer {
        input: Vec<char>,
        pos: usize,
    }

    impl Tokenizer {
        pub fn new(input: String) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        pub fn next_token(&mut self) -> Option<Token> {
            while self.pos < self.input.len() {
                match self.input[self.pos] {
                    '[' => {
                        self.pos += 1;
                        return Some(Token::OpenBracket);
                    }
                    ']' => {
                        self.pos += 1;
                        return Some(Token::CloseBracket);
                    }
                    '(' => {
                        self.pos += 1;
                        return Some(Token::OpenParen);
                    }
                    ')' => {
                        self.pos += 1;
                        return Some(Token::CloseParen);
                    }
                    ',' => {
                        self.pos += 1;
                        return Some(Token::Comma);
                    }
                    '=' => {
                        self.pos += 1;
                        return Some(Token::Equals);
                    }
                    '\'' => {
                        self.pos += 1;
                        let start = self.pos;
                        while self.pos < self.input.len() && self.input[self.pos] != '\'' {
                            self.pos += 1;
                        }
                        let end = self.pos;
                        self.pos += 1;
                        return Some(Token::StringLiteral(
                            self.input[start..end].iter().collect(),
                        ));
                    }
                    c if c.is_digit(10) || c == '.' => {
                        let start = self.pos;
                        while self.pos < self.input.len()
                            && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.')
                        {
                            self.pos += 1;
                        }
                        let end = self.pos;
                        return Some(Token::Number(
                            self.input[start..end]
                                .iter()
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                        ));
                    }
                    c if c.is_alphabetic() => {
                        let start = self.pos;
                        while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric()
                        {
                            self.pos += 1;
                        }
                        let end = self.pos;
                        return Some(Token::Identifier(self.input[start..end].iter().collect()));
                    }
                    _ => {
                        self.pos += 1;
                    }
                }
            }
            None
        }

        pub fn tokenize(&mut self) -> Vec<Token> {
            let mut tokens = Vec::new();
            while let Some(token) = self.next_token() {
                tokens.push(token);
            }
            tokens
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use crate::{
        bit::{AncillaQubit, Bit, BitOps, Clbit, Qubit},
        circuit_instruction::CircuitInstruction,
        gates::singleton::{HadamardGate, XGate, YGate, ZGate},
        init_operation,
        instruction::{Instruction, Operation, OperationPool, Unit},
        util::pool::Handle,
    };

    use regex::Regex;

    use super::{
        tokenizer::{Token, Tokenizer},
        QuantumCircuit,
    };

    pub struct Parser {
        tokens: Vec<Token>,
        pos: usize,
    }

    impl Parser {
        pub fn new(input: String) -> Self {
            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize();

            Self { tokens, pos: 0 }
        }

        pub fn parse(
            &mut self,
            gates: &mut OperationPool,
            gate_map: &mut HashMap<String, Handle<Box<dyn Operation>>>,
            qubits: &mut Vec<Qubit>,
            clbits: &mut Vec<Clbit>,
        ) -> Vec<CircuitInstruction> {
            let mut instructions = Vec::new();
            self.expect_token(Token::OpenBracket);
            while self.pos < self.tokens.len() {
                if let Some(token) = self.next_token() {
                    match token {
                        Token::Identifier(id) if id == "CircuitInstruction" => {
                            let instruction =
                                self.parse_circuit_instruction(gates, gate_map, qubits, clbits);
                            instructions.push(instruction);
                        }
                        Token::CloseBracket => break,
                        _ => {}
                    }
                }
            }
            instructions
        }

        fn parse_circuit_instruction(
            &mut self,
            gates: &mut OperationPool,
            gate_map: &mut HashMap<String, Handle<Box<dyn Operation>>>,
            qubits: &mut Vec<Qubit>,
            clbits: &mut Vec<Clbit>,
        ) -> CircuitInstruction {
            self.expect_token(Token::OpenParen);
            let (handle) = self.parse_operation(gates, gate_map);

            // Maybe no clone in the future? Overhead should be minimal tho
            let parsed_qubits: Vec<Qubit> = self
                .parse_group("qubits")
                .iter()
                .map(|bit| Qubit::from(bit.clone()))
                .collect();
            let parsed_clbits: Vec<Clbit> = self
                .parse_group("clbits")
                .iter()
                .map(|bit| Clbit::from(bit.clone()))
                .collect();
            self.expect_token(Token::CloseParen);

            for (qubit, clbit) in parsed_qubits.iter().zip(parsed_clbits.iter()) {
                if !qubits.contains(&qubit) {
                    qubits.push(qubit.clone());
                }
                if !clbits.contains(&clbit) {
                    clbits.push(clbit.clone());
                }
            }

            let qubit_indices = qubits.iter().map(|qubit| qubit.index()).collect();
            let clbit_indices = clbits.iter().map(|clbit| clbit.index()).collect();

            CircuitInstruction::new(handle, qubit_indices, clbit_indices)
        }

        fn parse_operation(
            &mut self,
            gates: &mut OperationPool,
            gate_map: &mut HashMap<String, Handle<Box<dyn Operation>>>,
        ) -> Handle<Box<dyn Operation>> {
            self.expect_token(Token::Identifier("operation".to_string()));
            self.expect_token(Token::Equals);
            self.expect_token(Token::Identifier("Instruction".to_string()));
            self.expect_token(Token::OpenParen);

            let name = self.parse_key_value("name", true).unwrap();

            // unused... will clean up parsing to ignore stuff like this
            let num_qubits: usize = self
                .parse_key_value("num_qubits", false)
                .unwrap()
                .parse()
                .unwrap();
            let num_clbits: usize = self
                .parse_key_value("num_clbits", false)
                .unwrap()
                .parse()
                .unwrap();

            let params: Vec<f64> = self
                .parse_key_value("params", false)
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            self.expect_token(Token::CloseParen);

            let instr = Instruction::new(params, None, Unit::DT);

            // TODO: In the future this should be pull from every struct in gates
            // and add them to the pool and the map. This is really ugly and should
            // be fixed. Currently, I'm having an issue coming up with a good
            // memory efficient way to iterate through all the gates. I was thinking
            // of using VecAny but it doesn't seem to do exactly what I need (also
            // there's memory overhead which is whatever but still not ideal). Maybe
            // there's a way I can just directly pull from the gates module...
            match name.as_str() {
                "x" => {
                    let handle = gates.add(&XGate::new(instr));
                    gate_map.insert("x".to_string(), handle);
                    handle
                }
                "y" => {
                    let handle = gates.add(&YGate::new(instr));
                    gate_map.insert("y".to_string(), handle);
                    handle
                }
                "z" => {
                    let handle = gates.add(&ZGate::new(instr));
                    gate_map.insert("z".to_string(), handle);
                    handle
                }
                "h" => {
                    let handle = gates.add(&HadamardGate::new(instr));
                    gate_map.insert("h".to_string(), handle);
                    handle
                }
                _ => panic!("Unexpected gate name: {:?}", name),
            }
        }

        /// Creates a group of bits
        fn parse_group(&mut self, group_name: &str) -> Vec<Bit> {
            let bit_pattern = Regex::new(r"(Qubit|Clbit|AncillaQubit)\((.*?)\)").unwrap();

            self.expect_token(Token::Comma);
            self.expect_token(Token::Identifier(group_name.to_string()));
            self.expect_token(Token::Equals);
            self.expect_token(Token::OpenParen);

            let mut group = Vec::new();

            while let Some(Token::Identifier(bit_type)) = self.next_token() {
                if bit_pattern.is_match(&bit_type) {
                    self.expect_token(Token::OpenParen);

                    let size = self.expect_number();

                    let name = self.expect_string();
                    // Parse the index
                    self.expect_token(Token::Comma);
                    let index = self.expect_number() as usize;
                    self.expect_token(Token::CloseParen);

                    match bit_type.as_str() {
                        "Qubit" => {
                            group.push(Bit::Qubit(Qubit::new(name, index)));
                        }
                        "Clbit" => {
                            group.push(Bit::Clbit(Clbit::new(name, index)));
                        }
                        "AncillaQubit" => {
                            group.push(Bit::AncillaQubit(AncillaQubit::new(name, index)));
                        }
                        _ => panic!("Unexpected bit type: {:?}", bit_type),
                    }
                } else {
                    panic!("Unexpected bit type format: {:?}", bit_type);
                }
            }

            self.expect_token(Token::CloseParen);
            self.expect_token(Token::Comma);
            group
        }

        /// Parses a single bit instruction
        fn parse_bit(&mut self) -> Bit {
            let bit_pattern =
                Regex::new(r"(QuantumRegister|ClassicalRegister|AncillaRegister)\((.*?)\)")
                    .unwrap();

            if let Some(Token::Identifier(bit_type)) = self.next_token() {
                if bit_pattern.is_match(&bit_type) {
                    self.expect_token(Token::OpenParen);

                    // unused
                    let size = self.expect_number();

                    let name = self.expect_string();
                    // Parse the index
                    self.expect_token(Token::Comma);
                    let index = self.expect_number() as usize;
                    self.expect_token(Token::CloseParen);

                    match bit_type.as_str() {
                        "QuantumRegister" => {
                            return Bit::Qubit(Qubit::new(name, index));
                        }
                        "ClassicalRegister" => {
                            return Bit::Clbit(Clbit::new(name, index));
                        }
                        "AncillaRegister" => {
                            return Bit::AncillaQubit(AncillaQubit::new(name, index));
                        }
                        _ => panic!("Unexpected bit type: {:?}", bit_type),
                    }
                } else {
                    panic!("Unexpected bit type format: {:?}", bit_type);
                }
            } else {
                panic!("Expected bit type but got None");
            }
        }

        fn parse_key_value(&mut self, key: &str, is_string: bool) -> Option<String> {
            self.expect_token(Token::Identifier(key.to_string()));
            self.expect_token(Token::Equals);
            if is_string {
                Some(self.expect_string())
            } else {
                self.next_token().map(|token| match token {
                    Token::Number(n) => n.to_string(),
                    Token::StringLiteral(s) => s,
                    Token::Identifier(id) => id,
                    _ => panic!("Unexpected token in key-value parsing"),
                })
            }
        }

        fn expect_token(&mut self, expected: Token) {
            let token = self.next_token().expect("Expected token but got None");
            if token != expected {
                panic!("Expected token {:?} but got {:?}", expected, token);
            }
        }

        fn expect_string(&mut self) -> String {
            if let Some(Token::StringLiteral(s)) = self.next_token() {
                s
            } else {
                panic!("Expected string but got None")
            }
        }

        fn expect_number(&mut self) -> f64 {
            if let Some(Token::Number(n)) = self.next_token() {
                n
            } else {
                panic!("Expected number but got None")
            }
        }

        fn next_token(&mut self) -> Option<Token> {
            if self.pos < self.tokens.len() {
                let token = self.tokens[self.pos].clone();
                self.pos += 1;
                Some(token)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Testing an X gate
    #[test]
    fn test_single_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let parser = QuantumCircuit::new(input.to_string()).unwrap();

        let instructions = parser.get_instructions();
        assert_eq!(instructions.len(), 1);
    }

    /// Testing an X and Y gate
    #[test]
    fn test_single_qubit_two_gates() {
        let input = "[CircuitInstruction(operation=Instruction(name='x', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='y', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(1, 'q'), 0),), clbits=())]";

        let parser = QuantumCircuit::new(input.to_string());
    }

    /// Testing a CNOT gate
    #[test]
    fn test_two_qubit_one_gate() {
        let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

        let parser = QuantumCircuit::new(input.to_string());
    }

    /// Testing the Bell state circuit
    #[test]
    fn test_bell_state() {
        let input = "[CircuitInstruction(operation=Instruction(name='h', num_qubits=1, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0),), clbits=()), CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(2, 'q'), 0), Qubit(QuantumRegister(2, 'q'), 1)), clbits=())]";

        let parser = QuantumCircuit::new(input.to_string());
    }

    /// Testing the classic |0> -> |000> QECC circuit
    #[test]
    fn test_naive_qecc() {
        let input = "[CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(3, 'q'), 0), Qubit(QuantumRegister(3, 'q'), 1)), clbits=()), CircuitInstruction(operation=Instruction(name='cx', num_qubits=2, num_clbits=0, params=[]), qubits=(Qubit(QuantumRegister(3, 'q'), 0), Qubit(QuantumRegister(3, 'q'), 2)), clbits=())]";

        let parser = QuantumCircuit::new(input.to_string());
    }
}
