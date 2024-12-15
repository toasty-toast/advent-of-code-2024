mod utils;

#[derive(Debug)]
struct Offset {
    row: i32,
    col: i32,
}

pub fn main() {
    let grid = parse_input();
    solve_part1(&grid);
    solve_part2(&grid);
}

fn solve_part1(grid: &Vec<Vec<char>>) {
    let offsets = vec![
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: 0, col: 1 },
            Offset { row: 0, col: 2 },
            Offset { row: 0, col: 3 },
        ], // right
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: 0, col: -1 },
            Offset { row: 0, col: -2 },
            Offset { row: 0, col: -3 },
        ], // left
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: 1, col: 0 },
            Offset { row: 2, col: 0 },
            Offset { row: 3, col: 0 },
        ], // down
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: 0 },
            Offset { row: -2, col: 0 },
            Offset { row: -3, col: 0 },
        ], // up
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: 1, col: 1 },
            Offset { row: 2, col: 2 },
            Offset { row: 3, col: 3 },
        ], // down-right
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: -1 },
            Offset { row: -2, col: -2 },
            Offset { row: -3, col: -3 },
        ], // up-left
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: 1, col: -1 },
            Offset { row: 2, col: -2 },
            Offset { row: 3, col: -3 },
        ], // down-left
        vec![
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: 1 },
            Offset { row: -2, col: 2 },
            Offset { row: -3, col: 3 },
        ], // up-right
    ];
    println!("Part 1: {}", count_words(&grid, "XMAS", &offsets));
}

fn solve_part2(grid: &Vec<Vec<char>>) {
    let offsets = vec![
        vec![
            Offset { row: -1, col: -1 },
            Offset { row: 1, col: -1 },
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: 1 },
            Offset { row: 1, col: 1 },
        ], // left to right
        vec![
            Offset { row: -1, col: 1 },
            Offset { row: 1, col: 1 },
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: -1 },
            Offset { row: 1, col: -1 },
        ], // right to left
        vec![
            Offset { row: -1, col: -1 },
            Offset { row: -1, col: 1 },
            Offset { row: 0, col: 0 },
            Offset { row: 1, col: -1 },
            Offset { row: 1, col: 1 },
        ], // up to down
        vec![
            Offset { row: 1, col: -1 },
            Offset { row: 1, col: 1 },
            Offset { row: 0, col: 0 },
            Offset { row: -1, col: -1 },
            Offset { row: -1, col: 1 },
        ], // down to up
    ];
    println!("Part 2: {}", count_words(&grid, "MMASS", &offsets));
}

fn count_words(grid: &Vec<Vec<char>>, word: &str, offsets: &Vec<Vec<Offset>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let words = get_words(&grid, i, j, offsets);
            count += words.iter().filter(|&w| w == word).count() as i32;
        }
    }
    return count;
}

fn get_words(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    offsets: &Vec<Vec<Offset>>,
) -> Vec<String> {
    let mut words = Vec::new();
    for offset_list in offsets {
        let mut chars: Vec<char> = Vec::new();
        for offset in offset_list {
            let new_row = row as i32 + offset.row;
            let new_col = col as i32 + offset.col;
            if new_row < 0
                || new_row >= grid.len() as i32
                || new_col < 0
                || new_col >= grid[0].len() as i32
            {
                break;
            }

            let letter: char = grid[new_row as usize][new_col as usize];
            chars.push(letter);
        }
        if chars.len() > 0 {
            words.push(chars.iter().collect());
        }
    }
    return words;
}

fn parse_input() -> Vec<Vec<char>> {
    return utils::read_puzzle_input()
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
}
