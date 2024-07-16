use crate::bit::Bit;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Register {
    size: Option<u32>,
    name: Option<String>,
    bits: Option<Bit>,
}
