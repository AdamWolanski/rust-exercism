pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().len();
    let num_str = num.to_string();

    let out = num_str.chars().fold(0, |acc, f| {
        acc + f.to_digit(10).unwrap().pow(num_len as u32)
    });

    return out == num
}