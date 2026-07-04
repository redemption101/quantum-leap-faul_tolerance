use crate::quantum::gates::Qubit;
use rand::Rng;

pub struct Measurement;

impl Measurement {
    pub fn measure(qubit: &mut Qubit) -> u8 {
        let prob_0 = qubit.alpha.norm_sqr();
        let mut rng = rand::thread_rng();
        let sample: f64 = rng.gen();

        if sample < prob_0 {
            qubit.alpha = nalgebra::Complex::new(1.0, 0.0);
            qubit.beta = nalgebra::Complex::new(0.0, 0.0);
            0
        } else {
            qubit.alpha = nalgebra::Complex::new(0.0, 0.0);
            qubit.beta = nalgebra::Complex::new(1.0, 0.0);
            1
        }
    }
}
