use std::collections::HashMap;

pub fn puzzle(input: &str) -> String {
    let len = String::from(input).lines().next().unwrap().len();
    let mut result = String::new();

    for col in 0..len {
        result.push(count(input, col));
    }

    result
}

fn count(input: &str, column: usize) -> char {
    let mut counts = HashMap::new();
    for line in input.split('\n') {
        println!("Got line {}", line);
        let column_char = line.chars().nth(column).unwrap();
        let count = counts.entry(column_char).or_insert(0);
        *count += 1;
    }

    *counts
         .iter()
         .min_by_key(|&(_key, value)| value)
         .unwrap()
         .0
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_column() {
        let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

        assert_eq!(super::count(input, 0), 'a');
        assert_eq!(super::count(input, 1), 'd');
        assert_eq!(super::count(input, 2), 'v');
        assert_eq!(super::puzzle(input), "advent");
    }
}
