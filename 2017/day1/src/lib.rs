pub fn puzzle(input: &str) -> u32 {
    let mut count: u32 = 0;
    let mut last_digit: u32 = 0;

    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    for digit in digits.iter() {
        if *digit == last_digit {
            count = count + *digit;
        }

        last_digit = *digit;
    }

    let first_digit: u32 = *digits.iter().next().unwrap();
    if first_digit == last_digit {
        count = count + first_digit;
    }

    return count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        assert_eq!(super::puzzle("1122"), 3);
        assert_eq!(super::puzzle("1111"), 4);
        assert_eq!(super::puzzle("1234"), 0);
        assert_eq!(super::puzzle("91212129"), 9);
    }
}
