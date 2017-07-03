extern crate md5;

#[derive(Debug)]
struct Entry {
    input: i32,
    hash: String,
}

pub fn puzzle(input: &str) -> String {
    (0..)
        .map(|i| {
                 Entry {
                     input: i,
                     hash: format!("{:x}", md5::compute(format!("{}{}", input, i))),
                 }
             })
        .filter(|entry| entry.hash.starts_with("00000"))
        .take(8)
        .map(|entry| entry.hash.chars().nth(5).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "abc";
        assert_eq!(puzzle(input), "18f47a30");
    }
}
