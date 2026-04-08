mod sumcheck;
mod table;
mod mle;
mod utils;
mod field;

use crate::sumcheck::prover::SumCheck;
use crate::sumcheck::verifier::SumCheckVerifier;
use crate::table::generate_table;

fn main() {
    println!("Hello, running Sum Check protocol now !");

    let n = 10;

    let table = generate_table(n);

    let prover = SumCheck::new(table, n);

    let verifier = SumCheckVerifier::new(n);

    verifier.verifier_game(&prover);
}
