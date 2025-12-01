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
                if amount >= num {
                    password += 1 + ((num - amount) / 100).abs() as usize;
                }
                num = (num - amount).rem_euclid(100);
            }
            "R" => {
                if num + amount >= 100 {
                    password += ((num + amount) / 100) as usize;
                }
                num = (num + amount).rem_euclid(100);
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
    fn left_logic(num: &mut isize, amount: isize) -> usize {
        let mut password = 0;

        if amount >= *num {
            password += 1 + ((*num - amount) / 100).abs() as usize;
        };
        *num = (*num - amount).rem_euclid(100);

        password
    }

    fn right_logic(num: &mut isize, amount: isize) -> usize {
        let mut password = 0;

        if *num + amount >= 100 {
            password += ((*num + amount) / 100).abs() as usize;
        };

        *num = (*num + amount).rem_euclid(100);

        password
    }

    #[test]
    fn test_left() {
        let mut num: isize = 50;
        let amount: isize = 10;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 40);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_left_to_zero() {
        let mut num: isize = 50;
        let amount: isize = 50;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left() {
        let mut num: isize = 50;
        let amount: isize = 70;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 80);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_to_original_num() {
        let mut num: isize = 50;
        let amount: isize = 100;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_to_less_than_original_num() {
        let mut num: isize = 50;
        let amount: isize = 120;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 30);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_left_twice() {
        let mut num: isize = 50;
        let amount: isize = 200;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_wraps_left_lands_on_zero() {
        let mut num: isize = 50;
        let amount: isize = 150;
        let password = left_logic(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_right() {
        let mut num: isize = 50;
        let amount: isize = 10;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 60);
        assert_eq!(password, 0);
    }

    #[test]
    fn test_right_to_zero() {
        let mut num: isize = 50;
        let amount: isize = 50;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right() {
        let mut num: isize = 50;
        let amount: isize = 70;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 20);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_to_original_num() {
        let mut num: isize = 50;
        let amount: isize = 100;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_to_more_than_original_num() {
        let mut num: isize = 50;
        let amount: isize = 120;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 70);
        assert_eq!(password, 1);
    }

    #[test]
    fn test_wraps_right_twice() {
        let mut num: isize = 50;
        let amount: isize = 200;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 50);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_wraps_right_lands_on_zero() {
        let mut num: isize = 50;
        let amount: isize = 150;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 0);
        assert_eq!(password, 2);
    }

    #[test]
    fn test_right_a_bunch() {
        let mut num: isize = 50;
        let amount: isize = (100 * 25) + 5;
        let password = right_logic(&mut num, amount);
        assert_eq!(num, 55);
        assert_eq!(password, 25);
    }
}
