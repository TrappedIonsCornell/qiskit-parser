use crate::bit::Bit;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// Register for the system
pub enum Register {
    QuantumRegister(QuantumRegister),
    ClassicalRegister(ClassicalRegister),
    AncillaRegister(AncillaRegister),
}

/// Register Operations
pub trait RegisterOps : From<Register> {
    fn new(size: Option<u32>, name: Option<String>, bits: Option<Bit>) -> Self;
    fn get_size(&self) -> Option<u32>;
    fn get_name(&self) -> Option<String>;
    fn get_bits(&self) -> Option<Bit>;
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// Quantum Register for the system
pub struct QuantumRegister {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// Classical Register for the system
pub struct ClassicalRegister {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// Ancilla Register for the system
pub struct AncillaRegister {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}

impl From<Register> for QuantumRegister {
    fn from(register: Register) -> Self {
        match register {
            Register::QuantumRegister(quantum_register) => quantum_register,
            _ => panic!("Cannot convert to QuantumRegister"),
        }
    }
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

impl From<Register> for ClassicalRegister {
    fn from(register: Register) -> Self {
        match register {
            Register::ClassicalRegister(classical_register) => classical_register,
            _ => panic!("Cannot convert to ClassicalRegister"),
        }
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

impl From<Register> for AncillaRegister {
    fn from(register: Register) -> Self {
        match register {
            Register::AncillaRegister(ancilla_register) => ancilla_register,
            _ => panic!("Cannot convert to AncillaRegister"),
        }
    }
}

impl RegisterOps for AncillaRegister {
    fn new(size: Option<u32>, name: Option<String>, bits: Option<Bit>) -> Self {
        AncillaRegister {
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