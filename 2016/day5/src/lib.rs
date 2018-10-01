extern crate md5;

pub fn puzzle(input: &str) -> String {
    let mut result = vec!['z'; 8];
    let mut found_positions = 0;
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap().to_digit(16).unwrap();
            if pos < 8 && result[pos as usize] == 'z' {
                result[pos as usize] = hash.chars().nth(6).unwrap();
                found_positions += 1;
                if found_positions == 8 {
                    break;
                }
            }
        }
    }
    return result.drain(..).collect::<String>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "abc";
        assert_eq!(puzzle(input), "05ace8e3");
    }
}
