use crate::field::F;
use ark_ff::{Zero, One};

fn optimized (r : &[F]) -> Vec<F> {
   
    let mut weights = vec![F::one()];

    for ri in r {

        let xi = F::one() - *ri;
        let yi = *ri;

        let mut new_weights = Vec::with_capacity(weights.len() * 2);

        for i in &weights {
            new_weights.push(*i * xi);
            new_weights.push(*i * yi);
        }
        weights = new_weights;
    }
    weights
}

pub fn mle (x : &[F], table : &Vec<F>) -> F {

    let weights = optimized(x);
    
    let mut result = F::zero();
    
    for (i, val) in table.iter().enumerate() {
        result += *val * weights[i];
    }
    result
}
