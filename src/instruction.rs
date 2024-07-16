pub struct Instruction {
    name: String,
    num_qubits: usize,
    num_clbits: usize,
    params: Vec<f64>, // temporary, will be replaced with a more complex type
    duration: f64,
    unit: String,
    label: Option<String>,
}