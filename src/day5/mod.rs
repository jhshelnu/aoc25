const INPUT: &str = include_str!("input.txt");

fn is_fresh(ranges: &[(usize, usize)], n: usize) -> bool {
    for range in ranges {
        if n >= range.0 && n <= range.1 {
            return true;
        }
    }

    false
}

fn solve_part_one() -> usize {
    let mut ranges = Vec::new();
    let mut parsing_ranges = true;

    let mut fresh_count = 0;

    INPUT.lines().for_each(|line| {
        if line == "" {
            parsing_ranges = false;
            return;
        }

        if parsing_ranges {
            let (low, high) = line
                .split_once('-')
                .expect("every line to be of the format \\d+-\\d+");

            let low: usize = low
                .parse()
                .expect("the lower bound of the range to be a number");

            let high: usize = high
                .parse()
                .expect("the upper bound of the range to be a number");

            ranges.push((low, high));
        } else {
            let n: usize = line.parse().expect("every ingredient ID to be a number");
            if is_fresh(&ranges, n) {
                fresh_count += 1;
            }
        }
    });

    fresh_count
}

fn merge(a: &(usize, usize), b: &(usize, usize)) -> Option<(usize, usize)> {
    if a.1 >= b.0 {
        let low = a.0;
        let high = a.1.max(b.1);
        Some((low, high))
    } else {
        None
    }
}

fn solve_part_two() -> usize {
    let mut parsing_ranges = true;

    let mut ranges = Vec::new();

    INPUT.lines().for_each(|line| {
        if line == "" {
            parsing_ranges = false;
            return;
        }

        if !parsing_ranges {
            return;
        }

        let (low, high) = line
            .split_once('-')
            .expect("every line to be of the format \\d+-\\d+");

        let low: usize = low
            .parse()
            .expect("the lower bound of the range to be a number");

        let high: usize = high
            .parse()
            .expect("the upper bound of the range to be a number");

        ranges.push((low, high));
    });

    ranges.sort_by_key(|(low, _)| *low);

    let mut merged_range = ranges[0];
    let mut fresh_count = 0;

    for range in ranges.iter().skip(1) {
        if let Some(merged) = merge(&merged_range, &range) {
            merged_range = merged;
        } else {
            fresh_count += merged_range.1 - merged_range.0 + 1;
            merged_range = range.clone();
        }
    }

    fresh_count += merged_range.1 - merged_range.0 + 1;

    fresh_count
}

pub fn solve() {
    println!("part 1: {}, part 2: {}", solve_part_one(), solve_part_two())
}

#[cfg(test)]
mod test {
    use super::merge;

    #[test]
    fn test() {
        assert_eq!(merge(&(1, 5), &(7, 9)), None);
        assert_eq!(merge(&(1, 5), &(5, 9)), Some((1, 9)));
        assert_eq!(merge(&(1, 5), &(3, 9)), Some((1, 9)));
        assert_eq!(merge(&(1, 5), &(1, 9)), Some((1, 9)));
        assert_eq!(merge(&(1, 5), &(1, 3)), Some((1, 5)));
    }
}
