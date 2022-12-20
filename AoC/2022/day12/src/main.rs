use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut dist: Vec<Vec<i32>> = vec![];
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
        dist.push(vec![-1; line.len()]);
    }
    let grid = grid;
    let start_pos = find_start(&grid, 'S');
    dist[start_pos.0][start_pos.1] = 0;
    to_visit.push_back(start_pos);

    while !to_visit.is_empty() {
        let pos = to_visit.pop_front().unwrap();
        if grid[pos.0][pos.1] == 'E' {
            return dist[pos.0][pos.1];
        }
        for new_pos in get_new_poses(pos) {
            if can_move(&grid, pos, new_pos) && dist[new_pos.0][new_pos.1] == -1 {
                to_visit.push_back(new_pos);
                dist[new_pos.0][new_pos.1] = dist[pos.0][pos.1] + 1
            }
        }
    }

    panic!("Where was 'E'?");
}

// Normally I like a delta array that I can loop through and add
// i.e. let deltas: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
// but this feels cleaner than isize <> usize addition/conversion
fn get_new_poses(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut new_poses = vec![];
    if pos.0 > 0 {
        new_poses.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        new_poses.push((pos.0, pos.1 - 1));
    }
    new_poses.push((pos.0 + 1, pos.1));
    new_poses.push((pos.0, pos.1 + 1));
    new_poses
}

fn can_move(grid: &Vec<Vec<char>>, pos: (usize, usize), new_pos: (usize, usize)) -> bool {
    if new_pos.0 >= grid.len() || new_pos.1 >= grid[0].len() {
        return false;
    };
    if pos.0 >= grid.len() || pos.1 >= grid[0].len() {
        return false;
    };
    (get_elevation(grid[new_pos.0][new_pos.1]) - get_elevation(grid[pos.0][pos.1])) <= 1
}

fn get_elevation(mut c: char) -> i32 {
    if c == 'E' {
        c = 'z';
    }
    if c == 'S' {
        c = 'a';
    }
    c as i32
}

fn find_start(grid: &Vec<Vec<char>>, start: char) -> (usize, usize) {
    for row_pos in 0..grid.len() {
        let row = &grid[row_pos];
        for col_pos in 0..row.len() {
            if row[col_pos] == start {
                return (row_pos, col_pos);
            }
        }
    }
    panic!("Where was 'S'?");
}

fn solve_part2(contents: &String) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut dist: Vec<Vec<i32>> = vec![];
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
        dist.push(vec![-1; line.len()]);
    }
    let grid = grid;
    let start_pos = find_start(&grid, 'E');
    dist[start_pos.0][start_pos.1] = 0;
    to_visit.push_back(start_pos);

    while !to_visit.is_empty() {
        let pos = to_visit.pop_front().unwrap();
        if grid[pos.0][pos.1] == 'a' {
            return dist[pos.0][pos.1];
        }
        for new_pos in get_new_poses(pos) {
            if can_move(&grid, new_pos, pos) && dist[new_pos.0][new_pos.1] == -1 {
                to_visit.push_back(new_pos);
                dist[new_pos.0][new_pos.1] = dist[pos.0][pos.1] + 1
            }
        }
    }

    panic!("Where was 'a'?");
}

fn _print_grid<T>(grid: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    for line in grid {
        for item in line {
            print!("{}", item);
        }
        println!();
    }
}
