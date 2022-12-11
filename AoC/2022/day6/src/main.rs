use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    solve(&contents, 4)
}

fn printVec(vec: &Vec<char>) {
    for c in vec {
        print!("{}", c);
    }
    println!();
}

fn solve_part2(contents: &String) -> i32 {
    solve(&contents, 14)
}

fn solve(contents: &String, len: usize) -> i32 {
    let mut unique: Vec<char> = Vec::new();
    let mut pos = 0;
    for c in contents.chars() {
        pos += 1;
        for idx in (0..unique.len()).rev() {
            if c == unique[idx] {
                unique = unique.split_off(idx + 1);
                break;
            }
        }
        unique.push(c);
        //printVec(&unique);
        if unique.len() == len {
            break;
        }
    }
    pos
}
