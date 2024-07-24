use ndarray::Array2;

use numpy::Complex64;
use std::fmt::Debug;

use std::{io, mem::ManuallyDrop};

use crate::gates::singleton::{XGate, YGate};
use crate::util::pool::{Handle, MMapArena, ARENA_SIZE_BYTES};

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
}


pub trait Operation {
    fn params(&self) -> &Vec<f64>;
    fn duration(&self) -> Option<f64>;
    fn unit(&self) -> Unit;
}

pub trait Gate {
    fn to_matrix(&self) -> Array2<Complex64>;
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
            *(next_item as *mut ManuallyDrop<Box<dyn Operation>>) = ManuallyDrop::new(Box::new(item));
            Handle::from(next_item)
        }
    }

    pub fn get(&self, handle: Handle<Box<dyn Operation>>) -> &dyn Operation {
        unsafe { &**handle.pointer() }
    }
}
