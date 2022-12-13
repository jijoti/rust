use std::fs;
use std::iter::{once, zip};

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    let grid = to_grid(contents);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut seen = init_seen(rows,cols);
    for row in 0..rows {
        mark_seen(&grid, &mut seen, once(row).cycle(), 0..cols);
        mark_seen(&grid, &mut seen, once(row).cycle(), (0..cols).rev());
    }
    for col in 0..cols {
        mark_seen(&grid, &mut seen, 0..rows, once(col).cycle());
        mark_seen(&grid, &mut seen, (0..rows).rev(), once(col).cycle());
    }
    seen.iter().flatten().sum()
}

fn init_seen(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut seen = vec![];
    for _n in 0..rows {
        seen.push(vec![0; cols]);
    }
    seen
} 

fn mark_seen(
    grid: &Vec<Vec<i8>>,
    seen: &mut Vec<Vec<i32>>,
    rows: impl Iterator<Item = usize>,
    cols: impl Iterator<Item = usize>,
) {
    let mut highest = -1;
    for (row, col) in zip(rows, cols) {
        if grid[row][col] > highest {
            seen[row][col] = 1;
            highest = grid[row][col];
        }
    }
}

fn solve_part2(contents: &String) -> i32 {
    let grid = to_grid(contents);
    grid_iter(grid.len(), grid[0].len())
        .map(|(row, col)| calc_scenic_score(&grid, row, col))
        .max()
        .unwrap()
}

fn grid_iter(rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..rows)
        .map(move |row| (0..cols).map(move |col| (row, col)))
        .flatten()
}

fn calc_scenic_score(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> i32 {
    let tree_height = grid[row][col];
    let same_row = once(row).cycle();
    let same_col = once(col).cycle();
    let right = col + 1..grid[row].len();
    let left = (0..col).rev();
    let down = row + 1..grid.len();
    let up = (0..row).rev();
    [
        calc_viewing_distance(&grid, tree_height, same_row.clone(), right),
        calc_viewing_distance(&grid, tree_height, same_row, left),
        calc_viewing_distance(&grid, tree_height, down, same_col.clone()),
        calc_viewing_distance(&grid, tree_height, up, same_col),
    ]
    .iter()
    .product()
}

fn calc_viewing_distance(
    grid: &Vec<Vec<i8>>,
    stop_height: i8,
    rows: impl Iterator<Item = usize>,
    cols: impl Iterator<Item = usize>,
) -> i32 {
    let mut viewing_distance = 0;
    for (row, col) in zip(rows, cols) {
        viewing_distance += 1;
        if grid[row][col] >= stop_height {
            break;
        }
    }
    viewing_distance
}

fn to_grid(contents: &String) -> Vec<Vec<i8>> {
    let mut grid: Vec<Vec<i8>> = vec![];
    for line in contents.lines() {
        let mut row: Vec<i8> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
        grid.push(row);
    }
    grid
}
