use ndarray::Array2;
use numpy::Complex64;
use std::fmt::Debug;

/// An instruction is defined by a name, the number of qubits/classical bits it
/// acts on, and other optional parameters.
///
/// Instructions act as the most general operation in a a quantum circuit, and
/// are converted into an InstructionType enum for further processing.
#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    name: String,
    num_qubits: usize,
    num_clbits: usize,
    params: Vec<f64>, // this can be replaced with a more complex type
    duration: Option<f64>,
    unit: Option<String>,
    label: Option<String>,
}

/// Operations are the most general trait for quantum operations. They provide
/// basic methods to retrieve information from an instruction.
pub trait Operation: Debug + PartialEq + Clone {
    fn name(&self) -> &str;
    fn num_qubits(&self) -> usize;
    fn num_clbits(&self) -> usize;
    fn params(&self) -> &[f64];
    fn duration(&self) -> Option<f64>;
    fn unit(&self) -> Option<&str>;
    fn label(&self) -> Option<&str>;
}

/// Contains the different types of instructions that can be used in a quantum
/// circuit.
#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    Gate(Instruction),
    Measurement(Instruction),
    Reset(Instruction),
    Barrier(Instruction),
    Delay(Instruction),
    Store(Instruction),
}

/// Gates are unitary operations that act on qubits. They can be converted into
/// a matrix representation.
pub trait Gate : Operation {
    fn to_matrix(&self) -> Array2<Complex64>;
}

/// Measurements are operations that measure the state of a qubit and transform
/// them into classical bits.
pub trait Measurement: Operation {}

/// Resets irreversibly set a qubit to the |0> state.
pub trait Reset: Operation {}

/// Barriers are operations that separate different parts of a quantum circuit.
pub trait Barrier: Operation {}

/// Delays are operations that represent a time delay in a quantum circuit.
pub trait Delay: Operation {}

/// Stores write a real-time classical expression to a storage location.
pub trait Store : Operation {}

impl Instruction {
    pub fn new(
        name: String,
        num_qubits: usize,
        num_clbits: usize,
        params: Vec<f64>,
        duration: Option<f64>,
        unit: Option<String>,
        label: Option<String>,
    ) -> Instruction {
        Instruction {
            name,
            num_qubits,
            num_clbits,
            params,
            duration,
            unit,
            label,
        }
    }
}

impl Operation for Instruction {
    fn name(&self) -> &str {
        &self.name
    }

    fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    fn num_clbits(&self) -> usize {
        self.num_clbits
    }

    fn params(&self) -> &[f64] {
        &self.params
    }

    fn duration(&self) -> Option<f64> {
        self.duration
    }

    fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl InstructionType {
    pub fn from(instruction: Instruction) -> InstructionType {
        // convert from the instruction name to the enum variant
        let allowed_gates: Vec<&str> = vec!["x", "y", "z", "h", "cx"];

        let name = instruction.name.as_str();
        match name {
            name if allowed_gates.contains(&name) => InstructionType::Gate(instruction),
            "measure" => InstructionType::Measurement(instruction),
            "reset" => InstructionType::Reset(instruction),
            "barrier" => InstructionType::Barrier(instruction),
            "delay" => InstructionType::Delay(instruction),
            "store" => InstructionType::Store(instruction),
            _ => unimplemented!(),
        }
    }
}

impl Operation for InstructionType {
    fn name(&self) -> &str {
        match self {
            InstructionType::Gate(inst) => inst.name(),
            InstructionType::Measurement(inst) => inst.name(),
            InstructionType::Reset(inst) => inst.name(),
            InstructionType::Barrier(inst) => inst.name(),
            InstructionType::Delay(inst) => inst.name(),
            InstructionType::Store(inst) => inst.name(),
        }
    }

    fn num_qubits(&self) -> usize {
        match self {
            InstructionType::Gate(inst) => inst.num_qubits(),
            InstructionType::Measurement(inst) => inst.num_qubits(),
            InstructionType::Reset(inst) => inst.num_qubits(),
            InstructionType::Barrier(inst) => inst.num_qubits(),
            InstructionType::Delay(inst) => inst.num_qubits(),
            InstructionType::Store(inst) => inst.num_qubits(),
        }
    }

    fn num_clbits(&self) -> usize {
        match self {
            InstructionType::Gate(inst) => inst.num_clbits(),
            InstructionType::Measurement(inst) => inst.num_clbits(),
            InstructionType::Reset(inst) => inst.num_clbits(),
            InstructionType::Barrier(inst) => inst.num_clbits(),
            InstructionType::Delay(inst) => inst.num_clbits(),
            InstructionType::Store(inst) => inst.num_clbits(),
        }
    }

    fn params(&self) -> &[f64] {
        match self {
            InstructionType::Gate(inst) => inst.params(),
            InstructionType::Measurement(inst) => inst.params(),
            InstructionType::Reset(inst) => inst.params(),
            InstructionType::Barrier(inst) => inst.params(),
            InstructionType::Delay(inst) => inst.params(),
            InstructionType::Store(inst) => inst.params(),
        }
    }

    fn duration(&self) -> Option<f64> {
        match self {
            InstructionType::Gate(inst) => inst.duration(),
            InstructionType::Measurement(inst) => inst.duration(),
            InstructionType::Reset(inst) => inst.duration(),
            InstructionType::Barrier(inst) => inst.duration(),
            InstructionType::Delay(inst) => inst.duration(),
            InstructionType::Store(inst) => inst.duration(),
        }
    }

    fn unit(&self) -> Option<&str> {
        match self {
            InstructionType::Gate(inst) => inst.unit(),
            InstructionType::Measurement(inst) => inst.unit(),
            InstructionType::Reset(inst) => inst.unit(),
            InstructionType::Barrier(inst) => inst.unit(),
            InstructionType::Delay(inst) => inst.unit(),
            InstructionType::Store(inst) => inst.unit(),
        }
    }

    fn label(&self) -> Option<&str> {
        match self {
            InstructionType::Gate(inst) => inst.label(),
            InstructionType::Measurement(inst) => inst.label(),
            InstructionType::Reset(inst) => inst.label(),
            InstructionType::Barrier(inst) => inst.label(),
            InstructionType::Delay(inst) => inst.label(),
            InstructionType::Store(inst) => inst.label(),
        }
    }
}
