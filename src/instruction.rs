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

pub trait Operation {
    fn new(
        name: String,
        num_qubits: usize,
        num_clbits: usize,
        params: Vec<f64>,
        duration: Option<f64>,
        unit: Option<String>,
        label: Option<String>,
    ) -> Instruction
    where
        Instruction : Sized,
    {
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
    fn name(&self) -> &str;
    fn num_qubits(&self) -> usize;
    fn num_clbits(&self) -> usize;
    fn params(&self) -> &[f64];
    fn duration(&self) -> Option<f64>;
    fn unit(&self) -> Option<&str>;
    fn label(&self) -> Option<&str>;
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

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    Gate(Instruction),
    Measurement(Instruction),
    Reset(Instruction),
    Barrier(Instruction),
    Delay(Instruction),
    Custom(Instruction), // For any other custom instruction types
}

impl Operation for InstructionType {
    fn name(&self) -> &str {
        match self {
            InstructionType::Gate(inst) => inst.name(),
            InstructionType::Measurement(inst) => inst.name(),
            InstructionType::Reset(inst) => inst.name(),
            InstructionType::Barrier(inst) => inst.name(),
            InstructionType::Delay(inst) => inst.name(),
            InstructionType::Custom(inst) => inst.name(),
        }
    }

    fn num_qubits(&self) -> usize {
        match self {
            InstructionType::Gate(inst) => inst.num_qubits(),
            InstructionType::Measurement(inst) => inst.num_qubits(),
            InstructionType::Reset(inst) => inst.num_qubits(),
            InstructionType::Barrier(inst) => inst.num_qubits(),
            InstructionType::Delay(inst) => inst.num_qubits(),
            InstructionType::Custom(inst) => inst.num_qubits(),
        }
    }

    fn num_clbits(&self) -> usize {
        match self {
            InstructionType::Gate(inst) => inst.num_clbits(),
            InstructionType::Measurement(inst) => inst.num_clbits(),
            InstructionType::Reset(inst) => inst.num_clbits(),
            InstructionType::Barrier(inst) => inst.num_clbits(),
            InstructionType::Delay(inst) => inst.num_clbits(),
            InstructionType::Custom(inst) => inst.num_clbits(),
        }
    }

    fn params(&self) -> &[f64] {
        match self {
            InstructionType::Gate(inst) => inst.params(),
            InstructionType::Measurement(inst) => inst.params(),
            InstructionType::Reset(inst) => inst.params(),
            InstructionType::Barrier(inst) => inst.params(),
            InstructionType::Delay(inst) => inst.params(),
            InstructionType::Custom(inst) => inst.params(),
        }
    }

    fn duration(&self) -> Option<f64> {
        match self {
            InstructionType::Gate(inst) => inst.duration(),
            InstructionType::Measurement(inst) => inst.duration(),
            InstructionType::Reset(inst) => inst.duration(),
            InstructionType::Barrier(inst) => inst.duration(),
            InstructionType::Delay(inst) => inst.duration(),
            InstructionType::Custom(inst) => inst.duration(),
        }
    }

    fn unit(&self) -> Option<&str> {
        match self {
            InstructionType::Gate(inst) => inst.unit(),
            InstructionType::Measurement(inst) => inst.unit(),
            InstructionType::Reset(inst) => inst.unit(),
            InstructionType::Barrier(inst) => inst.unit(),
            InstructionType::Delay(inst) => inst.unit(),
            InstructionType::Custom(inst) => inst.unit(),
        }
    }

    fn label(&self) -> Option<&str> {
        match self {
            InstructionType::Gate(inst) => inst.label(),
            InstructionType::Measurement(inst) => inst.label(),
            InstructionType::Reset(inst) => inst.label(),
            InstructionType::Barrier(inst) => inst.label(),
            InstructionType::Delay(inst) => inst.label(),
            InstructionType::Custom(inst) => inst.label(),
        }
    }
}
