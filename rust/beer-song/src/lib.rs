pub fn verse(n: u32) -> String {
    match n {
        0 => {
            format!(
                "No more bottles of beer on the wall, no more bottles of beer.\n\
                    Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            )
        }
        1 => {
            format!(
                "1 bottle of beer on the wall, 1 bottle of beer.\n\
                    Take it down and pass it around, no more bottles of beer on the wall.\n"
            )
        }
        2 => {
            format!(
                "2 bottles of beer on the wall, 2 bottles of beer.\n\
                    Take one down and pass it around, 1 bottle of beer on the wall.\n"
            )
        }
        _ => {
            format!(
                "{t1} bottles of beer on the wall, {t1} bottles of beer.\n\
                    Take one down and pass it around, {t2} bottles of beer on the wall.\n",
                t1 = n,
                t2 = n - 1
            )
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<String>>()
        .join("\n")
}
