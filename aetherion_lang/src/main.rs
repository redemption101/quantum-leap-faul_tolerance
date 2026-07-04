mod compiler;
mod typechecker;
mod quantum;

use quantum::gates::Qubit;
use quantum::measurement::Measurement;
use compiler::quantum_error_correction::LogicalQubit;

fn main() {
    println!("Aetherion Core Initialized: Rust Verifier & Quantum Engine Active.");

    let mut q1 = Qubit::new();
    q1.apply_hadamard();
    let classical_result = Measurement::measure(&mut q1);
    println!("Measurement collapsed superposition to classical bit: {}", classical_result);

    let mut q_logical = LogicalQubit::encode(q1);
    let verified = q_logical.correct_errors();
    println!("Shor Code error correction check verification status: {}", verified);
}
