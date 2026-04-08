use crate::field::F;

pub fn generate_table (n : usize) -> Vec<F>  {
    let size = 1 << n;

    let mut table = Vec::with_capacity(size);

    for i in 0..size {
        let val = F::from((i as i64) % 7);
        table.push(val);
    }
    table
}


