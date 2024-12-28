pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut xs = Vec::with_capacity(upper_bound as usize + 1);
    xs.extend((0..=upper_bound).map(Some));
    let upper = upper_bound as usize;

    (2..=upper)
        .filter_map(|i| {
            let p = xs[i].take()? as usize;
            (p * p..=upper).step_by(p).for_each(|j| xs[j] = None);
            Some(p as u64)
        })
        .collect()
}
