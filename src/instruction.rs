#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    name: String,
    num_qubits: usize,
    num_clbits: usize,
    params: Vec<f64>, // temporary, will be replaced with a more complex type
    duration: Option<f64>,
    unit: Option<String>,
    label: Option<String>,
}

impl Eq for Instruction {


}

impl Instruction {
    pub fn new(name: String, num_qubits: usize, num_clbits: usize, params: Vec<f64>, duration: Option<f64>, unit: Option<String>, label: Option<String>) -> Self {
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
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_num_qubits(&self) -> usize {
        self.num_qubits
    }
    pub fn get_num_clbits(&self) -> usize {
        self.num_clbits
    }
    pub fn get_params(&self) -> &Vec<f64> {
        &self.params
    }
    pub fn get_duration(&self) -> Option<f64> {
        self.duration
    }
    pub fn get_unit(&self) -> Option<&String> {
        self.unit.as_ref()
    }
    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}