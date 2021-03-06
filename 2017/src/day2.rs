pub fn puzzle(input: &str) -> u32 {
    String::from(input).lines().map(diff).sum()
}

fn diff(line: &str) -> u32 {
    let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    return numbers.iter().max().unwrap() - numbers.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";

        assert_eq!(super::puzzle(input), 18);
    }
    #[test]
    fn diff() {
        assert_eq!(super::diff("5 1 9 5"), 8);
        assert_eq!(super::diff("7 5 3"), 4);
        assert_eq!(super::diff("2 4 6 8"), 6);
        assert_eq!(super::diff("1 10"), 9);
    }

}
