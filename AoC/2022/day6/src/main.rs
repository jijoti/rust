use std::cmp::min;
use std::collections::HashMap;
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

// fn print_vec(vec: &Vec<char>) {
//     for c in vec {
//         print!("{}", c);
//     }
//     println!();
// }

fn solve_part2(contents: &String) -> i32 {
    solve(&contents, 14)
}

// O(len * contents.len())
// fn solve_old(contents: &String, len: usize) -> i32 {
//     let mut unique: Vec<char> = Vec::new();
//     let mut pos = 0;
//     for c in contents.chars() {
//         pos += 1;
//         for idx in (0..unique.len()).rev() {
//             if c == unique[idx] {
//                 unique = unique.split_off(idx + 1);
//                 break;
//             }
//         }
//         unique.push(c);
//         // print_vec(&unique);
//         if unique.len() == len {
//             break;
//         }
//     }
//     pos
// }

// O(contents.len())
fn solve(contents: &String, len: i32) -> i32 {
    let mut last_seen: HashMap<char, i32> = HashMap::new();
    let mut current_run = 0;
    let mut pos = 0;
    for c in contents.chars() {
        if current_run == len {
            break;
        }

        let last_seen_pos = if last_seen.contains_key(&c) {
            *last_seen.get(&c).unwrap()
        } else {
            -1
        };
        current_run = min(pos - last_seen_pos, current_run + 1);
        last_seen.insert(c, pos);
        pos += 1;
    }
    pos
}
