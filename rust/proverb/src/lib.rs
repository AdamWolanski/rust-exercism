pub fn build_proverb(list: &[&str]) -> String {

    let mut output = String::new();

    if !list.is_empty() {
        (0..list.len() - 1).for_each(|x| output += format!("For want of a {} the {} was lost.\n", list[x], list[x+1]).as_str());
        output += format!("And all for the want of a {}.", list[0]).as_str();
    }
    output
}