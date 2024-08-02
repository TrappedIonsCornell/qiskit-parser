use ndarray::Array2;

use numpy::Complex64;
use std::fmt::Debug;

pub type TimeDependentFn = fn(f64) -> Complex64;

/// Parts of a total Hamiltonian for a gate. This breaks up terms to easily
/// determine commutativity and other properties.
#[derive(Debug, PartialEq, Clone)]
pub struct HamiltonianComponent {
    time_fn: Option<TimeDependentFn>,
    constant: Option<Complex64>,
    operator: Array2<Complex64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hamiltonian {
    components: Vec<HamiltonianComponent>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeUnit {
    DT,
}

/// Contains all possible operations that can be applied to a quantum circuit.
/// This includes gates, delays, barriers, and measurements.
#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Gate(Gate),
    Delay(Delay),
    Barrier(Barrier),
    Measurement(Measurement),
}

/// A quantum gate that can be applied to a quantum circuit
#[derive(Debug, PartialEq, Clone)]
pub struct Gate {
    name: String,
    params: Vec<f64>,
    duration: Option<f64>,
    unit: TimeUnit,
    matrix: Array2<Complex64>,
    hamiltonian: Option<Hamiltonian>,
}

/// GateBuilder enables custom gate creation
#[derive(Debug, PartialEq, Clone)]
pub struct GateBuilder {
    name: Option<String>,
    params: Option<Vec<f64>>,
    duration: Option<f64>,
    unit: Option<TimeUnit>,
    matrix: Option<Array2<Complex64>>,
    hamiltonian: Option<Hamiltonian>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Delay {
    duration: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Barrier {
    // qubits: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Measurement {
    // qubits: Vec<usize>,
}

impl Gate {
    pub fn new(
        name: String,
        params: Vec<f64>,
        duration: Option<f64>,
        unit: TimeUnit,
        matrix: Array2<Complex64>,
        hamiltonian: Option<Hamiltonian>,
    ) -> Self {
        Gate {
            name,
            params,
            duration,
            unit,
            matrix,
            hamiltonian,
        }
    }

    /// If you want to update a prebuilt gate, utilize this method to update the
    /// gate and create a GateBuilder initialized with the gate's values.
    pub fn builder(&self) -> GateBuilder {
        GateBuilder::new()
            .name(self.name.clone())
            .params(self.params.clone())
            .duration(self.duration.unwrap())
            .unit(self.unit)
            .matrix(self.matrix.clone())
            .hamiltonian(self.hamiltonian.clone().unwrap())
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn params(&self) -> &Vec<f64> {
        &self.params
    }

    pub fn duration(&self) -> Option<f64> {
        self.duration
    }

    pub fn unit(&self) -> &TimeUnit {
        &self.unit
    }

    pub fn to_matrix(&self) -> Array2<Complex64> {
        self.matrix.clone()
    }
}

impl From<Operation> for Gate {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Gate(gate) => gate,
            _ => panic!("Operation is not a gate"),
        }
    }
}

impl Delay {
    pub fn new(duration: f64) -> Self {
        Delay { duration }
    }
}

impl Operation {
    pub fn id(&self) -> i64 {
        0
    }
}

impl GateBuilder {
    pub fn new() -> Self {
        GateBuilder {
            name: None,
            params: None,
            duration: None,
            unit: None,
            matrix: None,
            hamiltonian: None,
        }
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn params(mut self, params: Vec<f64>) -> Self {
        self.params = Some(params);
        self
    }

    fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    fn unit(mut self, unit: TimeUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    fn matrix(mut self, matrix: Array2<Complex64>) -> Self {
        self.matrix = Some(matrix);
        self
    }

    fn hamiltonian(mut self, hamiltonian: Hamiltonian) -> Self {
        self.hamiltonian = Some(hamiltonian);
        self
    }

    pub fn build(self) -> Gate {
        Gate {
            name: self.name.expect("Gate name not set"),
            params: self.params.unwrap(),
            duration: self.duration,
            unit: self.unit.unwrap(),
            matrix: self.matrix.expect("Gate matrix not set"),
            hamiltonian: self.hamiltonian,
        }
    }
}

impl HamiltonianComponent {
    pub fn new(
        time_fn: TimeDependentFn,
        constant: Complex64,
        operator: Array2<Complex64>,
    ) -> Self {
        HamiltonianComponent {
            time_fn: Some(time_fn),
            constant: Some(constant),
            operator,
        }
    }

    pub fn calculate(&self, t: f64) -> Array2<Complex64> {
        let time_fn = self.time_fn.expect("Time function not set");
        let constant = self.constant.expect("Constant not set");

        let time_dep = time_fn(t);
        let operator = self.operator.clone();

        operator * time_dep + constant
    }
}

impl Hamiltonian {
    pub fn new(components: Vec<HamiltonianComponent>) -> Self {
        Hamiltonian { components }
    }

    pub fn components(&self) -> &Vec<HamiltonianComponent> {
        &self.components
    }

    pub fn is_commutative(&self) -> bool {
        todo!()
    }
}
