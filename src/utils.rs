fn hyper_cube (n : usize) -> Vec<Vec<u8>> {

    let mut result = Vec::with_capacity(n);

    for i in (1 << n) {
        let b = [0u8; n];

        for j in 0..n {
            b[j] = ((i >> j) & 1);
        }
        result.push(b); 
    }
    result
}
