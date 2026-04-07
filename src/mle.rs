fn optimized (r : &[F]) -> Vec<F>{
   
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

fn mle (x : &[F], table : &[(Vec<u8>, F)]) -> F {

    let weights = optimized(x);
    
    let mut result = F::zero();
    
    for (i, (_, fb)) in table.iter().enumerate() {
        result += *fb * weights[i];
    }
    result
}
