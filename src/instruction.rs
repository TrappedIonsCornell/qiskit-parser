use ndarray::Array2;

use numpy::Complex64;
use std::fmt::Debug;

use std::{io, mem::ManuallyDrop};

use crate::util::pool::{Handle, MMapArena};

pub trait Operation {
    fn params(&self) -> &Vec<f64>;
    fn set_params(&mut self, params: Vec<f64>);

    fn duration(&self) -> Option<f64>;
    fn set_duration(&mut self, duration: Option<f64>);

    fn unit(&self) -> Unit;
    fn set_unit(&mut self, unit: Unit);
}

pub trait Gate {
    fn to_matrix(&self) -> Array2<Complex64>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit {
    DT,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    params: Vec<f64>,
    duration: Option<f64>,
    unit: Unit,
}

impl Instruction {
    pub fn new(params: Vec<f64>, duration: Option<f64>, unit: Unit) -> Self {
        Instruction {
            params,
            duration,
            unit,
        }
    }
}

impl Operation for Instruction {
    fn params(&self) -> &Vec<f64> {
        &self.params
    }

    fn duration(&self) -> Option<f64> {
        self.duration
    }

    fn unit(&self) -> Unit {
        self.unit
    }

    fn set_params(&mut self, params: Vec<f64>) {
        self.params = params;
    }

    fn set_duration(&mut self, duration: Option<f64>) {
        self.duration = duration;
    }

    fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }
}

pub struct OperationPool {
    arena: MMapArena<Box<dyn Operation>>,
}

impl OperationPool {
    pub fn new(size: usize) -> io::Result<Self> {
        Ok(Self {
            arena: MMapArena::new(size)?,
        })
    }

    pub fn add(&mut self, item: &dyn Operation) -> Handle<Box<dyn Operation>> {
        unsafe {
            let next_item = self.arena.alloc();
            *(next_item as *mut ManuallyDrop<Box<&dyn Operation>>) =
                ManuallyDrop::new(Box::new(item));
            Handle::from(next_item)
        }
    }

    pub fn get(&self, handle: Handle<Box<dyn Operation>>) -> &dyn Operation {
        unsafe { &**handle.pointer() }
    }
}

#[macro_export]
macro_rules! init_operation {
    ($struct_name:ident, $params:expr) => {{
        let mut instance = $struct_name {
            instruction: Instruction {
                params: vec![],
                duration: None,
                unit: Unit, // Initialize with a default value if needed
                // initialize other fields if needed...
            },
            // initialize other fields if needed...
        };
        instance.set_params($params);
        // instance.set_duration($duration);
        // instance.set_unit($unit);
        Box::new(instance) as Box<dyn Operation>
    }};
}