const INPUT: &str = include_str!("input.txt");

fn line_to_bank(line: &str) -> Vec<u8> {
    line.bytes().map(|n| n - b'0').collect::<Vec<u8>>()
}

pub fn max_joltage(bank: &[u8], n: usize) -> usize {
    let mut result: Vec<u8> = Vec::with_capacity(n);

    for (i, &digit) in bank.iter().enumerate() {
        while let Some(&last) = result.last() {
            let remaining_batteries = bank.len() - i;
            if last < digit && result.len() - 1 + remaining_batteries >= n {
                result.pop();
            } else {
                break;
            }
        }

        if result.len() < n {
            result.push(digit);
        }
    }

    let mut max_joltage = 0;
    for digit in result {
        max_joltage = max_joltage * 10 + digit as usize;
    }
    max_joltage
}

fn solve_part_one() -> usize {
    INPUT
        .lines()
        .map(|line| line_to_bank(line))
        .map(|bank| max_joltage(&bank, 2))
        .map(|joltage| joltage as usize)
        .sum()
}

fn solve_part_two() -> usize {
    INPUT
        .lines()
        .map(|line| line_to_bank(line))
        .map(|bank| max_joltage(&bank, 12))
        .map(|joltage| joltage as usize)
        .sum()
}

pub fn solve() {
    println!("part 1: {}, part 2: {}", solve_part_one(), solve_part_two())
}

#[cfg(test)]
mod test {
    use crate::day3::{line_to_bank, max_joltage};

    #[test]
    fn test_line_to_bank() {
        assert_eq!(line_to_bank("123"), vec![1, 2, 3]);
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage(&vec![1, 2, 3], 2), 23);
        assert_eq!(max_joltage(&vec![3, 2, 1], 2), 32);
    }
}
