use crate::field::F;
use crate::mle::mle;
use crate::utils::hyper_cube;
use ark_ff::{Zero, One};

pub struct SumCheck {
    pub table : Vec<F>,
    pub num_variables : usize,
}

impl SumCheck {
    pub fn new (table : Vec<F>, num_variables : usize)-> Self {
        Self {table, num_variables}
    }

    pub fn initial_compute(&self) -> F {
   
        let mut sum = F::zero();

        for val in &self.table {
            sum += *val;
        }
        sum
    } 

    pub fn compute_round_polynomial(&self, r_prefix : &[F]) -> (F,F) {

        let mut g0 = F::zero();
        let mut g1 = F::zero();

        let remaining_var = self.num_variables - r_prefix.len() - 1;

        for assignments in hyper_cube(remaining_var) {
                
            let mut point0 = Vec::new();
            point0.extend_from_slice(r_prefix);
            point0.push(F::zero());

            for bit in &assignments {
                point0.push(F::from(*bit as u64));
            }

            let mut point1 = Vec::new();
            point1.extend_from_slice(r_prefix);
            point1.push(F::one());

            for bit in &assignments {
                point1.push(F::from(*bit as u64));
            }


            g0 += mle(&point0, &self.table);
            g1 += mle(&point1, &self.table);
        }
        (g0, g1)
    }
}
