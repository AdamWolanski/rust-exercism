pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap()
}

fn is_prime(num: &u32) -> bool {
    !(2..num - 1).into_iter().any(|x| num % x == 0)
}
