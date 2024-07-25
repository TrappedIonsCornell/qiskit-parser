use ndarray::Array2;

use numpy::Complex64;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit {
    DT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Gate(Gate),
    Delay(Delay),
    Barrier(Barrier),
    Measurement(Measurement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Gate {
    name: String,
    params: Vec<f64>,
    duration: Option<f64>,
    unit: Unit,
    matrix: Array2<Complex64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GateBuilder {
    name: Option<String>,
    params: Option<Vec<f64>>,
    duration: Option<f64>,
    unit: Option<Unit>,
    matrix: Option<Array2<Complex64>>,
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
    pub fn new(name: String, params: Vec<f64>, duration: Option<f64>, unit: Unit, matrix: Array2<Complex64>) -> Self {
        Gate {
            name,
            params,
            duration,
            unit,
            matrix,
        }
    }

    pub fn builder() -> GateBuilder {
        GateBuilder::new()
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

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn to_matrix(&self) -> Array2<Complex64> {
        self.matrix.clone()
    }
}

impl Delay {
    pub fn new(duration: f64) -> Self {
        Delay { duration }
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

    fn unit(mut self, unit: Unit) -> Self {
        self.unit = Some(unit);
        self
    }

    fn matrix(mut self, matrix: Array2<Complex64>) -> Self {
        self.matrix = Some(matrix);
        self
    }

    pub fn build(self) -> Gate {
        Gate {
            name: self.name.expect("Gate name not set"),
            params: self.params.unwrap(),
            duration: self.duration,
            unit: self.unit.unwrap(),
            matrix: self.matrix.expect("Gate matrix not set"),
        }
    }
}