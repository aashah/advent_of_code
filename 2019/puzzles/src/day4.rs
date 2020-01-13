use std::collections::HashMap;

const NUM_A: u32 = 272091;
const NUM_B: u32 = 815432;

fn is_number_valid(num: u32) -> bool {
    let num_as_str = num.to_string();
    let mut last_char = '0';
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in num_as_str.chars() {
        if c < '0' || c > '9' {
            return false;
        }
        if c < last_char {
            return false;
        }

        let char_count = char_counts.entry(c).or_insert(0);
        *char_count += 1;

        last_char = c;
    }

    char_counts.iter().any(|(_, v)| *v == 2)
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
        assert_eq!(super::is_number_valid(123456), false);
        assert_eq!(super::is_number_valid(112233), true);
        assert_eq!(super::is_number_valid(123444), false);
        assert_eq!(super::is_number_valid(111122), true);
        assert_eq!(super::is_number_valid(123333), false);
    }
}
