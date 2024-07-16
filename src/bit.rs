use crate::register::Register;
use crate::register::ClassicalRegister;
use crate::register::QuantumRegister;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Bit {
    Qubit(Qubit),
    Clbit(Clbit),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Clbit {
    register: Box<ClassicalRegister>,
    index: usize,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Qubit {
    register: Box<QuantumRegister>,
    index: usize,
}

pub trait BitOps {
    fn new(register: Register, index: usize) -> Self;
    fn get_register(&self) -> Register;
    fn get_index(&self) -> usize;
}

impl BitOps for Qubit {
    fn new(register: Register, index: usize) -> Self {
        Qubit {
            register: Box::new(register.get_quantum_register().unwrap()),
            index,
        }
    }

    fn get_register(&self) -> Register {
        Register::QuantumRegister(*self.register.clone())
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

impl BitOps for Clbit {
    fn new(register: Register, index: usize) -> Self {
        Clbit {
            register: Box::new(register.get_classical_register().unwrap()),
            index,
        }
    }

    fn get_register(&self) -> Register {
        Register::ClassicalRegister(*self.register.clone())
    }

    fn get_index(&self) -> usize {
        self.index
    }
}