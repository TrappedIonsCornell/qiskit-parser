use crate::register::Register;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Bit {
    Qubit(Qubit),
    Clbit(Clbit),
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Clbit {
    register: Box<Register>,
    index: usize,

}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Qubit {
    register: Box<Register>,
    index: usize,
}
