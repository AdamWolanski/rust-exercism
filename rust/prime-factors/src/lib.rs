pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    while n > 1 {
        let x = (2..=n).find(|x| n % x == 0).unwrap();
        n /= x;
        result.push(x);
    }

    result
}