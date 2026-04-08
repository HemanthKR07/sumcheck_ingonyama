use crate::sumcheck::prover::SumCheck;
use crate::field::F;
use crate::mle::mle;
use rand::Rng;
use ark_ff::One;

pub struct SumCheckVerifier {
    pub num_variables : usize,
}

impl SumCheckVerifier {

    pub fn new(num_variables : usize) -> Self {
        Self{num_variables}
    }

    pub fn verifier_game(&self, prover : &SumCheck) {

        let mut claim = prover.initial_compute();

        let mut random_variables : Vec<F> = Vec::new();
       
        let mut random_gen = rand::thread_rng();

        for round in 0..self.num_variables {
            let (g0, g1) = prover.compute_round_polynomial(&random_variables);
            println!("Round : {} - g0 : {:?} and g1 : {:?}", round, g0, g1);

            if g0 + g1 != claim {
                println!("Verification failed, g0 and g1 poly eval didnt match the previous sum !!.");
                panic!("Failed to verify round : {}", round);
            }

            let r_var = F::from(random_gen.gen_range(0..7));

            random_variables.push(r_var);

            println!("Next random variable at {} : {:?}", round, r_var);
            
            claim = (F::one() - r_var) * g0 + r_var * g1;

            println!("Claim for next round : {:?}", claim);
        }

        let final_check = mle(&random_variables, &prover.table);

        if final_check != claim {
            println!("Final evaluation failed : {:?}", final_check);
        } else {
            println!("Sumcheck passed : {:?}", final_check);
        }
    }
}
