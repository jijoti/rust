use std::fs;

fn main() {
    let contents = fs::read_to_string(".\\input.txt")
    .expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    let split = contents.split("\r\n");
    let mut current: i32 = 0;
    for s in split {
        current += get_score(s);
    }
    current
}

fn get_score(hand: &str) -> i32 {
    match hand {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => panic!(),
    }
}
// 0 3 6
// X A 1 Rock
// Y B 2 Paper
// Z C 3 scissors
fn get_score2(hand: &str) -> i32 {
    match hand {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => panic!(),
    }
}

fn solve_part2(contents: &String) -> i32 {
    let split = contents.split("\r\n");
    let mut current: i32 = 0;
    for s in split {
        current += get_score2(s);
    }
    current
}
