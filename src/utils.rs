pub fn hyper_cube (n : usize) -> Vec<Vec<u8>> {

    let size = 1 << n;
    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        let mut b = vec![0u8; n];

        for j in 0..n {
            b[j] = ((i >> j) & 1) as u8;
        }
        result.push(b); 
    }
    result
}
