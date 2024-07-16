use crate::register::ClassicalRegister;
use crate::register::QuantumRegister;
use crate::register::AncillaRegister;
use crate::register::Register;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Bit {
    Qubit(Qubit),
    Clbit(Clbit),
    AncillaQubit(AncillaQubit),
}

pub trait BitOps {
    fn new(register: Register, index: usize) -> Self;
    fn get_register(&self) -> Register;
    fn get_index(&self) -> usize;
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

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct AncillaQubit {
    register: Box<AncillaRegister>,
    index: usize,
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

impl BitOps for AncillaQubit {
    fn new(register: Register, index: usize) -> Self {
        AncillaQubit {
            register: Box::new(register.get_ancilla_register().unwrap()),
            index,
        }
    }

    fn get_register(&self) -> Register {
        Register::AncillaRegister(*self.register.clone())
    }

    fn get_index(&self) -> usize {
        self.index
    }
}