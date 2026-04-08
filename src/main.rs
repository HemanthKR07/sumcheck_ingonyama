mod sumcheck;
mod table;
mod mle;
mod utils;
mod field;

use crate::sumcheck::prover::SumCheck;
use crate::sumcheck::verifier::SumCheckVerifier;
use crate::table::generate_table;
use std::time::Instant;

fn main() {
    println!("Hello, running Sum Check protocol now !");

    let n = 12;

    let mut start = Instant::now();
    
    let table = generate_table(n);

    let mut duration = start.elapsed();
    
    println!("Time for table creation : {:?}", duration);

    let iteration = 1;

    start = Instant::now();

    let prover = SumCheck::new(table, n);
    let verifier = SumCheckVerifier::new(n);
    
    for _ in 0..iteration {
        verifier.verifier_game(&prover);
    }

    duration = start.elapsed();
    
    println!("Time for {} rounds : {:?}", iteration, duration);
}
