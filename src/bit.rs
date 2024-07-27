#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A generic bit
pub enum Bit {
    Qubit(Qubit),
    Clbit(Clbit),
    AncillaQubit(AncillaQubit),
}

pub trait BitOps : From<Bit> {
    fn new(name: String, index: usize) -> Self;
    fn name(&self) -> String;
    fn index(&self) -> usize;
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A classical bit
pub struct Clbit {
    name: String,
    index: usize,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// A quantum bit
pub struct Qubit {
    name: String,
    index: usize,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
/// An ancilla quantum bit (i.e. a quantum bit that is not part of the main
/// register)
pub struct AncillaQubit {
    name: String,
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

impl BitOps for Qubit {
    fn new(name: String, index: usize) -> Self {
        Qubit { name, index }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl BitOps for Clbit {
    fn new(name: String, index: usize) -> Self {
        Clbit { name, index }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl BitOps for AncillaQubit {
    fn new(name: String, index: usize) -> Self {
        AncillaQubit { name, index }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn index(&self) -> usize {
        self.index
    }
}