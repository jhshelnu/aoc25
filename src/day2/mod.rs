use fancy_regex::Regex;
use once_cell::sync::Lazy;

const INPUT: &str = include_str!("input.txt");

fn is_valid_part_one(n: usize) -> bool {
    let n = n.to_string();
    let mid = n.len() / 2;
    n[0..mid] != n[mid..]
}

fn solve_part_one() -> usize {
    INPUT
        .split(',')
        .flat_map(|range| {
            let (low, high) = range
                .split_once('-')
                .expect("every range to be separated by a dash");

            let low: usize = low
                .parse()
                .expect("the lower bound of the range to an integer");

            let high: usize = high
                .trim_end() // file ends in '\n'
                .parse()
                .expect("the upper bound of the range to an integer");

            low..=high
        })
        .filter(|n: &usize| !is_valid_part_one(*n))
        .sum()
}

static INVALID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)\1+$").unwrap());

fn is_valid_part_two(n: usize) -> bool {
    let n = n.to_string();
    !INVALID_PATTERN.is_match(&n).unwrap()
}

fn solve_part_two() -> usize {
    INPUT
        .split(',')
        .flat_map(|range| {
            let (low, high) = range
                .split_once('-')
                .expect("every range to be separated by a dash");

            let low: usize = low
                .parse()
                .expect("the lower bound of the range to an integer");

            let high: usize = high
                .trim_end() // file ends in '\n'
                .parse()
                .expect("the upper bound of the range to an integer");

            low..=high
        })
        .filter(|n: &usize| !is_valid_part_two(*n))
        .sum()
}

pub fn solve() {
    println!("part 1: {}, part 2: {}", solve_part_one(), solve_part_two())
}

#[cfg(test)]
mod test {
    use super::{is_valid_part_one, is_valid_part_two};

    #[test]
    fn test_validity_part_one() {
        assert!(!is_valid_part_one(99));
        assert!(is_valid_part_one(100));
        assert!(is_valid_part_one(101));
        assert!(!is_valid_part_one(456456));
        assert!(is_valid_part_one(4564563));
        assert!(is_valid_part_one(111));
    }

    #[test]
    fn test_validity_part_two() {
        assert!(!is_valid_part_two(99));
        assert!(is_valid_part_two(100));
        assert!(is_valid_part_two(101));
        assert!(!is_valid_part_two(456456));
        assert!(is_valid_part_two(4564563));
        assert!(!is_valid_part_two(111));
    }
}
