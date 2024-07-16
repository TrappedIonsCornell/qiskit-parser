use crate::bit::Bit;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Register {
    QuantumRegister(QuantumRegister),
    ClassicalRegister(ClassicalRegister),
}

impl Register {
    pub fn get_quantum_register(&self) -> Option<QuantumRegister> {
        match self {
            Register::QuantumRegister(register) => Some(register.clone()),
            _ => None,
        }
    }
    pub fn get_classical_register(&self) -> Option<ClassicalRegister> {
        match self {
            Register::ClassicalRegister(register) => Some(register.clone()),
            _ => None,
        }
    }
}

pub trait RegisterOps {
    fn new(size: Option<u32>, name: Option<String>, bits: Option<Bit>) -> Self;
    fn get_size(&self) -> Option<u32>;
    fn get_name(&self) -> Option<String>;
    fn get_bits(&self) -> Option<Bit>;
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct QuantumRegister {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ClassicalRegister {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}

impl RegisterOps for QuantumRegister {
    fn new(size: Option<u32>, name: Option<String>, bits: Option<Bit>) -> Self {
        QuantumRegister {
            size,
            name,
            bits,
        }
    }

    fn get_size(&self) -> Option<u32> {
        self.size
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_bits(&self) -> Option<Bit> {
        self.bits.clone()
    }
}

impl RegisterOps for ClassicalRegister {
    fn new(size: Option<u32>, name: Option<String>, bits: Option<Bit>) -> Self {
        ClassicalRegister {
            size,
            name,
            bits,
        }
    }

    fn get_size(&self) -> Option<u32> {
        self.size
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_bits(&self) -> Option<Bit> {
        self.bits.clone()
    }
}