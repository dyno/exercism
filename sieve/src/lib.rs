pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut xs = vec![true; upper_bound as usize + 1];

    xs[0] = false;
    xs[1] = false;
    
    let sqrt_bound = (upper_bound as f64).sqrt() as u64;
    for i in 2..=sqrt_bound {
        if xs[i as usize] {
            for j in (i * i..=upper_bound).step_by(i as usize) {
                xs[j as usize] = false;
            }
        }
    }
    
    xs.iter()
        .enumerate()
        .filter_map(|(i, &x)| if x { Some(i as u64) } else { None })
        .collect()
}
