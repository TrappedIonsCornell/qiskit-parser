use crate::register::{AncillaRegister, ClassicalRegister, QuantumRegister, Register};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A generic bit
pub enum Bit {
    Qubit(Qubit),
    Clbit(Clbit),
    AncillaQubit(AncillaQubit),
}

pub trait BitOps : From<Bit> {
    fn new(register: Register, index: usize) -> Self;
    fn register(&self) -> Register;
    fn index(&self) -> usize;
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A classical bit
pub struct Clbit {
    register: Box<ClassicalRegister>,
    index: usize,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A quantum bit
pub struct Qubit {
    register: Box<QuantumRegister>,
    index: usize,
}

impl From<Bit> for Qubit {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Qubit(qubit) => qubit,
            _ => panic!("Cannot convert to Qubit"),
        }
    }
}

impl From<Bit> for Clbit {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Clbit(clbit) => clbit,
            _ => panic!("Cannot convert to Clbit"),
        }
    }
}

impl From<Bit> for AncillaQubit {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::AncillaQubit(ancilla_qubit) => ancilla_qubit,
            _ => panic!("Cannot convert to AncillaQubit"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// An ancilla quantum bit (i.e. a quantum bit that is not part of the main
/// register)
pub struct AncillaQubit {
    register: Box<AncillaRegister>,
    index: usize,
}

impl BitOps for Qubit {
    fn new(register: Register, index: usize) -> Self {
        Qubit {
            register: Box::new(QuantumRegister::from(register)),
            index,
        }
    }

    fn register(&self) -> Register {
        Register::QuantumRegister(*self.register.clone())
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl BitOps for Clbit {
    fn new(register: Register, index: usize) -> Self {
        Clbit {
            register: Box::new(ClassicalRegister::from(register)),
            index,
        }
    }

    fn register(&self) -> Register {
        Register::ClassicalRegister(*self.register.clone())
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl BitOps for AncillaQubit {
    fn new(register: Register, index: usize) -> Self {
        AncillaQubit {
            register: Box::new(AncillaRegister::from(register)),
            index,
        }
    }

    fn register(&self) -> Register {
        Register::AncillaRegister(*self.register.clone())
    }

    fn index(&self) -> usize {
        self.index
    }
}
