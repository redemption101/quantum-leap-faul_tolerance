use nalgebra::{Matrix2, Complex};

#[derive(Debug, Clone)]
pub struct Qubit {
    pub alpha: Complex<f64>,
    pub beta: Complex<f64>,
}

impl Qubit {
    pub fn new() -> Self {
        Self {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    pub fn apply_hadamard(&mut self) {
        let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
        let h_matrix = Matrix2::new(
            Complex::new(inv_sqrt2, 0.0), Complex::new(inv_sqrt2, 0.0),
            Complex::new(inv_sqrt2, 0.0), Complex::new(-inv_sqrt2, 0.0),
        );
        
        let state_vector = nalgebra::Vector2::new(self.alpha, self.beta);
        let new_state = h_matrix * state_vector;
        
        self.alpha = new_state[0];
        self.beta = new_state[1];
    }
}
