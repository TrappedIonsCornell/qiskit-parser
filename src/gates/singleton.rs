use ndarray::Array2;
use numpy::Complex64 as c64;

use crate::operations::{Gate, TimeUnit};

macro_rules! call_all_functions {
    ($($func:ident),*) => {
        $(pub mod $func;)*
    };
}

pub fn hadamard() -> Gate {
    let factor = 1.0 / 2.0_f64.sqrt();
    let matrix = Array2::from_shape_vec(
        (2, 2),
        vec![
            c64::new(factor, 0.0),
            c64::new(factor, 0.0),
            c64::new(factor, 0.0),
            c64::new(-factor, 0.0),
        ],
    )
    .unwrap();
    Gate::new("h".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn x() -> Gate {
    let matrix = Array2::from_shape_vec(
        (2, 2),
        vec![
            c64::new(0.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
        ],
    )
    .unwrap();
    Gate::new("x".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn y() -> Gate {
    let matrix = Array2::from_shape_vec(
        (2, 2),
        vec![
            c64::new(0.0, 0.0),
            c64::new(0.0, -1.0),
            c64::new(0.0, 1.0),
            c64::new(0.0, 0.0),
        ],
    )
    .unwrap();
    Gate::new("y".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn z() -> Gate {
    let matrix = Array2::from_shape_vec(
        (2, 2),
        vec![
            c64::new(1.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(0.0, 0.0),
            c64::new(-1.0, 0.0),
        ],
    )
    .unwrap();
    Gate::new("z".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}

pub fn cx() -> Gate {
    let matrix = Array2::from_shape_vec(
        (4, 4),
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
    )
    .unwrap();

    Gate::new("cx".to_string(), vec![], None, TimeUnit::DT, matrix, None)
}
