use crate::{
    bit::{Clbit, Qubit},
    circuit_instruction::CircuitInstruction,
    instruction::Instruction,
    util::pool::{AsPool, Pool},
};

pub struct QuantumCircuit {
    instr: Vec<CircuitInstruction>,
    gates: Pool<Instruction, ()>,
    qubits: Vec<Qubit>,
    clbits: Vec<Clbit>,
}

impl QuantumCircuit {
    pub fn new(input: String) -> Self {
        let instr = parser::parse(input);
        let gates = Pool::new().expect("Failed to create pool");
        let qubits = vec![];
        let clbits = vec![];

        QuantumCircuit {
            instr,
            gates,
            qubits,
            clbits,
        }
    }

    pub fn add_gate(&mut self, gate: Instruction) {
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
    use crate::{
        bit::{AncillaQubit, Bit, BitOps, Clbit, Qubit},
        circuit_instruction::CircuitInstruction,
        instruction::Instruction,
        register::{AncillaRegister, ClassicalRegister, QuantumRegister, Register},
    };

    use regex::Regex;

    use super::tokenizer::{Token, Tokenizer};

    struct Parser {
        tokens: Vec<Token>,
        pos: usize,
    }

    impl Parser {
        pub fn new(input: String) -> Self {
            let mut tokenizer = Tokenizer::new(input);
            let tokens = tokenizer.tokenize();
            Self { tokens, pos: 0 }
        }

        pub fn parse(&mut self) -> Vec<CircuitInstruction> {
            let mut instructions = Vec::new();
            self.expect_token(Token::OpenBracket);
            while self.pos < self.tokens.len() {
                if let Some(token) = self.next_token() {
                    match token {
                        Token::Identifier(id) if id == "CircuitInstruction" => {
                            let instruction = self.parse_circuit_instruction();
                            instructions.push(instruction);
                        }
                        Token::CloseBracket => break,
                        _ => {}
                    }
                }
            }
            instructions
        }

        fn parse_circuit_instruction(&mut self) -> CircuitInstruction {
            self.expect_token(Token::OpenParen);
            let operation = self.parse_operation();
            let qubits = self.parse_group("qubits");
            let clbits = self.parse_group("clbits");
            self.expect_token(Token::CloseParen);
        }

        fn parse_operation(&mut self) -> Instruction {
            self.expect_token(Token::Identifier("operation".to_string()));
            self.expect_token(Token::Equals);
            self.expect_token(Token::Identifier("Instruction".to_string()));
            self.expect_token(Token::OpenParen);

            let name = self.parse_key_value("name", true).unwrap();
            let num_qubits = self
                .parse_key_value("num_qubits", false)
                .unwrap()
                .parse()
                .unwrap();
            let num_clbits = self
                .parse_key_value("num_clbits", false)
                .unwrap()
                .parse()
                .unwrap();
            let params = self
                .parse_key_value("params", false)
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            self.expect_token(Token::CloseParen);
            Instruction {
                name,
                num_qubits,
                num_clbits,
                params,
            }
        }

        /// Creates a group of bits
        fn parse_group(&mut self, group_name: &str) -> Vec<Bit> {
            self.expect_token(Token::Comma);
            self.expect_token(Token::Identifier(group_name.to_string()));
            self.expect_token(Token::Equals);
            self.expect_token(Token::OpenParen);

            let mut group = Vec::new();
            while let Some(token) = self.next_token() {
                match token {
                    Token::Identifier(id) if id == "Qubit" => {
                        self.expect_token(Token::OpenParen);
                        group.push(self.parse_bit());
                    }
                    Token::CloseParen => break,
                    _ => panic!("Unexpected token in group: {:?}", token),
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

                    // Parse the register
                    let size = self.expect_number() as usize;
                    let name = self.expect_string();

                    // Parse the index
                    self.expect_token(Token::Comma);
                    let index = self.expect_number() as usize;
                    self.expect_token(Token::CloseParen);

                    match bit_type.as_str() {
                        "QuantumRegister" => {
                            let register =
                                Register::QuantumRegister(QuantumRegister::new(size, name));
                            return Bit::Qubit(Qubit::new(register, index));
                        }
                        "ClassicalRegister" => {
                            let register =
                                Register::ClassicalRegister(ClassicalRegister::new(size, name));
                            return Bit::Clbit(Clbit::new(register, index));
                        }
                        "AncillaRegister" => {
                            let register =
                                Register::AncillaRegister(AncillaRegister::new(size, name));
                            return Bit::AncillaQubit(AncillaQubit::new(register, index));
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

        let parser = QuantumCircuit::new(input.to_string());
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
