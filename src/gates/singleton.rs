use nalgebra::base::DMatrix;
use crate::c64;

use crate::operations::{Gate, TimeUnit};

macro_rules! call_all_functions {
    ($($func:ident),*) => {
        $(pub mod $func;)*
    };
}

pub fn hadamard() -> Gate {
    let factor = 1.0 / 2.0_f64.sqrt();
    let matrix = DMatrix::from_vec(
        2,
        2,
        vec![
            c64::new(factor, 0.0),
            c64::new(factor, 0.0),
            c64::new(factor, 0.0),
            c64::new(-factor, 0.0),
        ],
    );
    Gate::new("h".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn x() -> Gate {
    let matrix = DMatrix::from_vec(
        2,
        2,
        vec![
            c64::new(0.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
        ],
    );
    Gate::new("x".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn y() -> Gate {
    let matrix = DMatrix::from_vec(
        2,
        2,
        vec![
            c64::new(0.0, 0.0),
            c64::new(0.0, -1.0),
            c64::new(0.0, 1.0),
            c64::new(0.0, 0.0),
        ],
    );
    Gate::new("y".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn z() -> Gate {
    let matrix = DMatrix::from_vec(
        2,
        2,
        vec![
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(-1.0, 0.0),
        ],
    );
    Gate::new("z".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn cx() -> Gate {
    let matrix = DMatrix::from_vec(
        4,
        4,
        vec![
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
        ],
    );
    Gate::new("cx".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}
