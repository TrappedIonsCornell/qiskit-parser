use ndarray::Array2;

use numpy::Complex64;
use std::fmt::Debug;

use std::{io, mem::ManuallyDrop};

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

    pub fn add(&mut self, item: Box<dyn Operation>) -> Handle<Box<dyn Operation>> {
        unsafe {
            let next_item = self.arena.alloc();
            *(next_item as *mut ManuallyDrop<Box<dyn Operation>>) = ManuallyDrop::new(item);
            Handle::from(next_item)
        }
    }

    pub fn get(&self, handle: Handle<Box<dyn Operation>>) -> &dyn Operation {
        unsafe { &**handle.pointer() }
    }
}

fn main() -> io::Result<()> {
    let mut pool = OperationPool::new(ARENA_SIZE_BYTES)?;

    // let a = StructA { data: 42 };
    // let b = StructB { data: 84 };

    // let handle_a = pool.add(Box::new(a));
    // let handle_b = pool.add(Box::new(b));

    // let item_a = pool.get(handle_a);
    // let item_b = pool.get(handle_b);

    // item_a.do_something();
    // item_b.do_something();

    Ok(())
}
