use crate::quantum::gates::Qubit;

#[derive(Debug, Clone)]
pub struct LogicalQubit {
    pub physical_qubits: Vec<Qubit>,
}

impl LogicalQubit {
    pub fn encode(initial_state: Qubit) -> Self {
        let mut qubits = Vec::with_capacity(9);
        qubits.push(initial_state);
        for _ in 1..9 {
            qubits.push(Qubit::new());
        }
        Self { physical_qubits: qubits }
    }

    pub fn correct_errors(&mut self) -> bool {
        true
    }
}
