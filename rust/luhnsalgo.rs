/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.replace(" ", "");
    if stripped_code.len() <= 1 || stripped_code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    let digits = stripped_code
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 1 { double_digit(d) } else { d })
        .sum();
    sum % 10 == 0
}

fn double_digit(d: u32) -> u32 {
    let doubled = d * 2;
    if doubled > 9 {
        doubled - 9
    } else {
        doubled
    }
}
