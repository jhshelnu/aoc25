const INPUT: &str = include_str!("input.txt");

fn is_forklift_accessible(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let x = x as isize;
    let y = y as isize;

    let mut adjacent_rolls = 0;
    for i in y - 1..=y + 1 {
        for j in x - 1..=x + 1 {
            // don't count a roll as being adjacent to itself
            if i == y && j == x {
                continue;
            }

            // don't try to index out of bounds vertically
            if i < 0 || i as usize >= board.len() {
                continue;
            }

            // don't try to index out of bounds horizontally
            let row = &board[y as usize];
            if j < 0 || j as usize >= row.len() {
                continue;
            }

            // found an adjacent roll
            if board[i as usize][j as usize] == '@' {
                adjacent_rolls += 1;
            }
        }
    }

    adjacent_rolls < 4
}

fn solve_part_one() -> usize {
    let board = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '@' && is_forklift_accessible(&board, x, y) {
                result += 1;
            }
        }
    }

    result
}

// returns how many rolls were removed
fn remove_accessible_rolls(board: &mut Vec<Vec<char>>) -> usize {
    let mut rolls_removed = 0;

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '@' && is_forklift_accessible(&board, x, y) {
                board[y][x] = '.';
                rolls_removed += 1;
            }
        }
    }

    rolls_removed
}

fn solve_part_two() -> usize {
    let mut board = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_rolls_removed = 0;

    loop {
        let rolls_removed = remove_accessible_rolls(&mut board);
        total_rolls_removed += rolls_removed;

        if rolls_removed == 0 {
            break;
        }
    }

    total_rolls_removed
}

pub fn solve() {
    println!("part 1: {}, part 2: {}", solve_part_one(), solve_part_two())
}
