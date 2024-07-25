// pub mod singleton;

use operation_macro::call_all_functions;

#[call_all_functions]
pub mod singleton {
    use ndarray::Array2;
    use numpy::Complex64;

    use crate::operations::{Gate, Unit};

    pub fn hadamard() -> Gate {
        let factor = 1.0 / 2.0_f64.sqrt();
        let matrix = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(factor, 0.0),
                Complex64::new(factor, 0.0),
                Complex64::new(factor, 0.0),
                Complex64::new(-factor, 0.0),
            ],
        )
        .unwrap();
        Gate::new("Hadamard".to_string(), vec![], None, Unit::DT, matrix)
    }

    pub fn x() -> Gate {
        let matrix = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap();
        Gate::new("Pauli-X".to_string(), vec![], None, Unit::DT, matrix)
    }

    pub fn y() -> Gate {
        let matrix = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(0.0, 1.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap();
        Gate::new("Pauli-Y".to_string(), vec![], None, Unit::DT, matrix)
    }

    pub fn z() -> Gate {
        let matrix = Array2::from_shape_vec(
            (2, 2),
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0),
            ],
        )
        .unwrap();
        Gate::new("Pauli-Z".to_string(), vec![], None, Unit::DT, matrix)
    }
}
