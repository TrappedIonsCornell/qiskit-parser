/// Generic Tokens for parsing a Qiskit circuit.
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

/// Tokenizer for parsing a Qiskit circuit.
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
                c if c.is_alphabetic() || c == '_' => {
                    let start = self.pos;
                    while self.pos < self.input.len()
                        && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_')
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
