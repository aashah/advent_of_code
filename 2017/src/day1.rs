pub fn puzzle(input: &str) -> u32 {
    let mut count: u32 = 0;
    let mut first_digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let second_digits = first_digits.split_off(input.len() / 2);

    for (c, d) in first_digits.iter().zip(second_digits.iter()) {
        if c == d {
            count = count + (c * 2);
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        assert_eq!(super::puzzle("1212"), 6);
        assert_eq!(super::puzzle("1221"), 0);
        assert_eq!(super::puzzle("123123"), 12);
        assert_eq!(super::puzzle("12131415"), 4);
    }
}
