pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut xs: Vec<_> = (0..=upper_bound).map(Option::from).collect();
    let upper_bound = upper_bound as usize;

    (2..=upper_bound)
        .filter_map(|i| {
            let p = xs[i].take()? as usize;
            (p * p..=upper_bound).step_by(p).for_each(|j| xs[j] = None);
            Some(p as u64)
        })
        .collect()
}
