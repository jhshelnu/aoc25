const INPUT: &str = include_str!("input.txt");

fn solve_part_one() -> usize {
    let mut num = 50;
    let mut password = 0;

    let instructions = INPUT.lines().map(|line| {
        let (direction, amount) = line.split_at(1);
        let amount = amount
            .parse::<isize>()
            .expect("every amount to be a number");
        (direction, amount)
    });

    for (direction, amount) in instructions {
        match direction {
            "L" => {
                num = (num - amount).rem_euclid(100);
            }
            "R" => {
                num = (num + amount).rem_euclid(100);
            }
            _ => {
                unreachable!("file only contains L or R for the direction");
            }
        };

        if num == 0 {
            password += 1;
        }
    }

    password
}

fn move_left(num: &mut isize, amount: isize) -> usize {
    let distance_to_next_zero = if *num == 0 { 100 } else { *num };

    *num = (*num - amount).rem_euclid(100);

    if amount < distance_to_next_zero {
        return 0;
    }

    1 + ((amount - distance_to_next_zero) / 100) as usize
}

fn move_right(num: &mut isize, amount: isize) -> usize {
    let distance_to_next_zero = if *num == 0 { 100 } else { 100 - *num };

    *num = (*num + amount).rem_euclid(100);

    if amount < distance_to_next_zero {
        return 0;
    };

    1 + ((amount - distance_to_next_zero) / 100) as usize
}

fn solve_part_two() -> usize {
    let mut num = 50;
    let mut password = 0;

    let instructions = INPUT.lines().map(|line| {
        let (direction, amount) = line.split_at(1);
        let amount = amount
            .parse::<isize>()
            .expect("every amount to be a number");
        (direction, amount)
    });

    for (direction, amount) in instructions {
        match direction {
            "L" => {
                password += move_left(&mut num, amount);
            }
            "R" => {
                password += move_right(&mut num, amount);
            }
            _ => {
                unreachable!("file only contains L or R for the direction");
            }
        };
    }

    password
}

pub fn solve() {
    println!("part 1: {}, part 2: {}", solve_part_one(), solve_part_two())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_left_no_movement() {
        let mut num: isize = 50;
        let amount: isize = 0;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_left() {
        let mut num: isize = 50;
        let amount: isize = 10;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 40);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_left_to_zero() {
        let mut num: isize = 50;
        let amount: isize = 50;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left() {
        let mut num: isize = 50;
        let amount: isize = 70;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 80);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_to_original_num() {
        let mut num: isize = 50;
        let amount: isize = 100;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_to_less_than_original_num() {
        let mut num: isize = 50;
        let amount: isize = 120;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 30);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_twice() {
        let mut num: isize = 50;
        let amount: isize = 200;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_wraps_left_lands_on_zero() {
        let mut num: isize = 50;
        let amount: isize = 150;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_right_no_movement() {
        let mut num: isize = 50;
        let amount: isize = 0;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_right() {
        let mut num: isize = 50;
        let amount: isize = 10;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 60);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_right_to_zero() {
        let mut num: isize = 50;
        let amount: isize = 50;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right() {
        let mut num: isize = 50;
        let amount: isize = 70;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 20);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_to_original_num() {
        let mut num: isize = 50;
        let amount: isize = 100;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_to_more_than_original_num() {
        let mut num: isize = 50;
        let amount: isize = 120;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 70);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_twice() {
        let mut num: isize = 50;
        let amount: isize = 200;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_wraps_right_lands_on_zero() {
        let mut num: isize = 50;
        let amount: isize = 150;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_right_a_bunch() {
        let mut num: isize = 50;
        let amount: isize = (100 * 25) + 5;
        let password = move_right(&mut num, amount);
        assert_eq!(num, 55);
        assert_eq!(password, 25);
    }

    #[test]
    fn left_starting_at_zero() {
        let mut num: isize = 0;
        let amount: isize = 5;
        let password = move_left(&mut num, amount);
        assert_eq!(num, 95);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_sample_input() {
        let mut num: isize = 50;
        let mut password = 0;

        password += move_left(&mut num, 68);
        assert_eq!(num, 82);
        assert_eq!(password, 1);

        password += move_left(&mut num, 30);
        assert_eq!(num, 52);
        assert_eq!(password, 1);

        password += move_right(&mut num, 48);
        assert_eq!(num, 0);
        assert_eq!(password, 2);

        password += move_left(&mut num, 5);
        assert_eq!(num, 95);
        assert_eq!(password, 2);

        password += move_right(&mut num, 60);
        assert_eq!(num, 55);
        assert_eq!(password, 3);

        password += move_left(&mut num, 55);
        assert_eq!(num, 0);
        assert_eq!(password, 4);

        password += move_left(&mut num, 1);
        assert_eq!(num, 99);
        assert_eq!(password, 4);

        password += move_left(&mut num, 99);
        assert_eq!(num, 0);
        assert_eq!(password, 5);

        password += move_right(&mut num, 14);
        assert_eq!(num, 14);
        assert_eq!(password, 5);

        password += move_left(&mut num, 82);
        assert_eq!(num, 32);
        assert_eq!(password, 6);
    }
}
