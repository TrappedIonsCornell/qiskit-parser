use std::collections::HashMap;

use nalgebra::base::DMatrix;
use crate::c64;

/// TODO: Migrate to a standard parser library instead of a custom one (didn't realized these existed before lol)
use crate::{
    bit::{AncillaQubit, Bit, BitOps, Clbit, Qubit},
    circuit_instruction::CircuitInstruction,
    operations::{Delay, Gate, Operation, TimeUnit},
};

use crate::gates::singleton;

use super::tokenizer::{Token, Tokenizer};

macro_rules! insert_gates {
    ($map:expr, $($gate:ident),*) => {
        $(
            $map.insert(
                singleton::$gate().name().to_string(),
                singleton::$gate().to_matrix(),
            );
        )*
    };
}

/// Reads in the tokenized Qiskit circuit data and parses it into a QuantumCircuit object.
/// This should not be instantiated by itself, but rather through the QuantumCircuit::new() method
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    mtx_map: HashMap<String, DMatrix<c64>>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut mtx_map: HashMap<String, DMatrix<c64>> = HashMap::new();

        insert_gates!(mtx_map, hadamard, x, y, z, cx);

        Self {
            tokens,
            pos: 0,
            mtx_map,
        }
    }

    pub fn parse(
        &mut self,
        gates: &mut Vec<Operation>,
        qubits: &mut Vec<Qubit>,
        clbits: &mut Vec<Clbit>,
    ) -> Vec<CircuitInstruction> {
        let mut instructions = Vec::new();
        self.expect_token(Token::OpenBracket);
        while self.pos < self.tokens.len() {
            if let Some(token) = self.next_token() {
                match token {
                    Token::Identifier(id) if id == "CircuitInstruction" => {
                        let instruction = self.parse_circuit_instruction(gates, qubits, clbits);
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
        operations: &mut Vec<Operation>,
        qubits: &mut Vec<Qubit>,
        clbits: &mut Vec<Clbit>,
    ) -> CircuitInstruction {
        self.expect_token(Token::OpenParen);
        let operation = self.parse_operation(operations);

        // Maybe no clone in the future? Overhead should be minimal tho
        let parsed_qubits: Vec<Qubit> = self
            .parse_bits("qubits")
            .iter()
            .map(|bit| Qubit::from(bit.clone()))
            .collect();
        let parsed_clbits: Vec<Clbit> = self
            .parse_bits("clbits")
            .iter()
            .map(|bit| Clbit::from(bit.clone()))
            .collect();
        self.expect_token(Token::CloseParen);

        for qubit in parsed_qubits.iter() {
            if !qubits.contains(&qubit) {
                qubits.push(qubit.clone());
            }
        }
        for clbit in parsed_clbits.iter() {
            if !clbits.contains(&clbit) {
                clbits.push(clbit.clone());
            }
        }

        let qubit_indices = qubits.iter().map(|qubit| qubit.index()).collect();
        let clbit_indices = clbits.iter().map(|clbit| clbit.index()).collect();

        CircuitInstruction::new(operation, qubit_indices, clbit_indices)
    }

    fn parse_operation(&mut self, operations: &mut Vec<Operation>) -> Operation {
        self.expect_token(Token::Identifier("operation".to_string()));
        self.expect_token(Token::Equals);
        self.expect_token(Token::Identifier("Instruction".to_string()));
        self.expect_token(Token::OpenParen);

        let name = self.parse_key_value("name", true).unwrap();

        // unused... will clean up parsing to ignore stuff like this
        self.parse_key_value("num_qubits", false);
        self.parse_key_value("num_clbits", false);

        let params: Vec<f64> = self.parse_params();

        self.expect_token(Token::CloseParen);

        let operation = match name.as_str() {
            // gates have custom names
            _ => {
                let mtx = match self.mtx_map.get(&name) {
                    Some(_) => self.mtx_map.get(&name).unwrap().clone(),
                    None => {
                        let mtx = DMatrix::zeros(2, 2);
                        self.mtx_map.insert(name.clone(), mtx.clone());
                        mtx
                    }
                };
                let operation = Operation::Gate(Gate::new(
                    name.clone(),
                    params,
                    None,
                    TimeUnit::DT,
                    mtx,
                    None,
                ));
                operations.push(operation);
                operations.last().unwrap().clone()
            }
        };
        operation
    }

    fn parse_bits(&mut self, group_name: &str) -> Vec<Bit> {
        self.expect_token(Token::Comma);
        self.expect_token(Token::Identifier(group_name.to_string()));
        self.expect_token(Token::Equals);
        self.expect_token(Token::OpenParen);

        let mut group = Vec::new();

        while let Some(Token::Identifier(bit_type)) = self.next_token() {
            if bit_type == "Qubit" || bit_type == "Clbit" || bit_type == "AncillaQubit" {
                self.expect_token(Token::OpenParen);
                self.expect_token(Token::Identifier("QuantumRegister".to_string()));
                self.expect_token(Token::OpenParen);
                self.expect_number();
                self.expect_token(Token::Comma);

                let name = self.expect_string();
                self.expect_token(Token::CloseParen);
                self.expect_token(Token::Comma);

                let index = self.expect_number() as usize;
                self.expect_token(Token::CloseParen);
                self.expect_token(Token::Comma);
                match bit_type.as_str() {
                    "Qubit" => {
                        group.push(Bit::Qubit(Qubit::new(name, index)));
                        self.expect_token(Token::CloseParen);
                        break;
                    }
                    "Clbit" => {
                        group.push(Bit::Clbit(Clbit::new(name, index)));
                        self.expect_token(Token::CloseParen);
                        break;
                    }
                    "AncillaQubit" => {
                        group.push(Bit::AncillaQubit(AncillaQubit::new(name, index)));
                        break;
                    }
                    _ => panic!("Unexpected bit type: {:?}", bit_type),
                }
            } else {
                panic!("Unexpected bit type format: {:?}", bit_type);
            }
        }
        group
    }

    fn parse_params(&mut self) -> Vec<f64> {
        self.expect_token(Token::Comma);
        self.expect_token(Token::Identifier("params".to_string()));
        self.expect_token(Token::Equals);
        self.expect_token(Token::OpenBracket);

        let mut params = Vec::new();

        // Catches no parameters
        if self.tokens[self.pos] == Token::CloseBracket {
            self.pos += 1;
            return params;
        }

        while let Some(Token::Number(n)) = self.next_token() {
            params.push(n);
        }

        self.expect_token(Token::CloseBracket);
        params
    }

    fn parse_key_value(&mut self, key: &str, is_string: bool) -> Option<String> {
        if self.tokens[self.pos] == Token::Comma {
            self.pos += 1;
        }
        self.expect_token(Token::Identifier(key.to_string()));
        self.expect_token(Token::Equals);
        if is_string {
            Some(self.expect_string())
        } else {
            self.next_token().map(|token| match token {
                Token::Number(n) => n.to_string(),
                Token::StringLiteral(s) => s,
                Token::Identifier(id) => id,
                _ => panic!("Unexpected token in key-value parsing: {:?}", token),
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
