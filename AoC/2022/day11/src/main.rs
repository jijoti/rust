use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

// Hand parsed the problem
fn solve_part1(_contents: &String) -> i32 {
    let mut items = vec![
        vec![65, 78],
        vec![54, 78, 86, 79, 73, 64, 85, 88],
        vec![69, 97, 77, 88, 87],
        vec![99],
        vec![60, 57, 52],
        vec![91, 82, 85, 73, 84, 53],
        vec![88, 74, 68, 56],
        vec![54, 82, 72, 71, 53, 99, 67],
    ];
    let mut counts = vec![0; 8];
    let funcs: Vec<fn(i32) -> (i32, usize)> = vec![
        apply_0, apply_1, apply_2, apply_3, apply_4, apply_5, apply_6, apply_7,
    ];
    for _ in 0..20 {
        for monkey in 0..8 {
            for item in items[monkey].clone() {
                counts[monkey] += 1;
                let (new_worry, new_monkey) = funcs[monkey](item);
                items[new_monkey].push(new_worry);
            }
            items[monkey] = vec![];
        }
    }
    counts.sort();
    counts.iter().rev().take(2).product()
}

fn apply_0(item: i32) -> (i32, usize) {
    if item % 5 == 0 {
        (item, 2)
    } else {
        (item, 3)
    }
}
fn apply_1(item: i32) -> (i32, usize) {
    let new_item = (item + 8) / 3;
    if new_item % 11 == 0 {
        (new_item, 4)
    } else {
        (new_item, 7)
    }
}
fn apply_2(item: i32) -> (i32, usize) {
    let new_item = (item + 2) / 3;
    if new_item % 2 == 0 {
        (new_item, 5)
    } else {
        (new_item, 3)
    }
}
fn apply_3(item: i32) -> (i32, usize) {
    let new_item = (item + 4) / 3;
    if new_item % 13 == 0 {
        (new_item, 1)
    } else {
        (new_item, 5)
    }
}
fn apply_4(item: i32) -> (i32, usize) {
    let new_item = (item * 19) / 3;
    if new_item % 7 == 0 {
        (new_item, 7)
    } else {
        (new_item, 6)
    }
}
fn apply_5(item: i32) -> (i32, usize) {
    let new_item = (item + 5) / 3;
    if new_item % 3 == 0 {
        (new_item, 4)
    } else {
        (new_item, 1)
    }
}
fn apply_6(item: i32) -> (i32, usize) {
    let new_item = (item * item) / 3;
    if new_item % 17 == 0 {
        (new_item, 0)
    } else {
        (new_item, 2)
    }
}
fn apply_7(item: i32) -> (i32, usize) {
    let new_item = (item + 1) / 3;
    if new_item % 19 == 0 {
        (new_item, 6)
    } else {
        (new_item, 0)
    }
}

// Pretty sure it would have fit in i64
// 2^64 ~= (10^3)^6 = 10^18, 
// but we're moding each step by only 10^7
// largest operation is square so only need 10^14
fn solve_part2(_contents: &String) -> i128 {
    let mut items = vec![
        vec![65, 78],
        vec![54, 78, 86, 79, 73, 64, 85, 88],
        vec![69, 97, 77, 88, 87],
        vec![99],
        vec![60, 57, 52],
        vec![91, 82, 85, 73, 84, 53],
        vec![88, 74, 68, 56],
        vec![54, 82, 72, 71, 53, 99, 67],
    ];
    let mut counts = vec![0; 8];
    let funcs: Vec<fn(i128) -> (i128, usize)> = vec![
        apply_0_2, apply_1_2, apply_2_2, apply_3_2, apply_4_2, apply_5_2, apply_6_2, apply_7_2,
    ];
    for _ in 0..10000 {
        for monkey in 0..8 {
            for item in items[monkey].clone() {
                counts[monkey] += 1;
                let (new_worry, new_monkey) = funcs[monkey](item);
                items[new_monkey].push(new_worry);
            }
            items[monkey] = vec![];
        }
    }
    counts.sort();
    counts.iter().rev().take(2).product()
}
// 9699690 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19
// These are the first 8 primes and what we need
// to mod against to decide next monkey. Since they
// are prime we can mod by 9699690 and know we'll
// get the result when moding by any of its prime factors.
fn apply_0_2(item: i128) -> (i128, usize) {
    let new_item = (item * 3) % 9699690;
    if new_item % 5 == 0 {
        (new_item, 2)
    } else {
        (new_item, 3)
    }
}
fn apply_1_2(item: i128) -> (i128, usize) {
    let new_item = (item + 8) % 9699690;
    if new_item % 11 == 0 {
        (new_item, 4)
    } else {
        (new_item, 7)
    }
}
fn apply_2_2(item: i128) -> (i128, usize) {
    let new_item = (item + 2) % 9699690;
    if new_item % 2 == 0 {
        (new_item, 5)
    } else {
        (new_item, 3)
    }
}
fn apply_3_2(item: i128) -> (i128, usize) {
    let new_item = (item + 4) % 9699690;
    if new_item % 13 == 0 {
        (new_item, 1)
    } else {
        (new_item, 5)
    }
}
fn apply_4_2(item: i128) -> (i128, usize) {
    let new_item = (item * 19) % 9699690;
    if new_item % 7 == 0 {
        (new_item, 7)
    } else {
        (new_item, 6)
    }
}
fn apply_5_2(item: i128) -> (i128, usize) {
    let new_item = (item + 5) % 9699690;
    if new_item % 3 == 0 {
        (new_item, 4)
    } else {
        (new_item, 1)
    }
}
fn apply_6_2(item: i128) -> (i128, usize) {
    let new_item = (item * item) % 9699690;
    if new_item % 17 == 0 {
        (new_item, 0)
    } else {
        (new_item, 2)
    }
}
fn apply_7_2(item: i128) -> (i128, usize) {
    let new_item = (item + 1) % 9699690;
    if new_item % 19 == 0 {
        (new_item, 6)
    } else {
        (new_item, 0)
    }
}
