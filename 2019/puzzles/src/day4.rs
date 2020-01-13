const NUM_A: u32 = 272091;
const NUM_B: u32 = 815432;

fn is_number_valid(num: u32) -> bool {
    let num_as_str = num.to_string();

    let mut last_char = '0';
    let mut found_duplicate = false;

    for c in num_as_str.chars() {
        if c < '0' || c > '9' {
            return false;
        }
        if c < last_char {
            return false;
        }

        if c == last_char {
            found_duplicate = true;
        }

        last_char = c;
    }

    found_duplicate
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut count: u32 = 0;

    for i in NUM_A..NUM_B {
        if is_number_valid(i) {
            count += 1;
        }
    }

    println!("Answer: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_is_valid() {
        assert_eq!(super::is_number_valid(111111), true);
        assert_eq!(super::is_number_valid(223450), false);
        assert_eq!(super::is_number_valid(123789), false);
    }
}
