pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut asd = Vec::new();

    factors.iter().filter(|y| **y > 0).for_each(|x| {
        (*x..limit).step_by(*x as usize).for_each(|f| asd.push(f));
    });

    asd.sort();
    asd.dedup();

    asd.iter().sum()
}
