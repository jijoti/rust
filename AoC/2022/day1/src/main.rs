use std::fs;

fn main() {
    let contents = fs::read_to_string("C:\\Users\\Joshua\\.vscode\\rust\\AoC\\day1\\input.txt")
    .expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    let split = contents.split("\r\n");
    let mut current: i32 = 0;
    let mut max: i32 = 0;
    for s in split {
        if s == "" {
            max = if current > max { current } else { max };
            current = 0;
        } else {
            current += s.parse::<i32>().expect("Not a number!");
        }
    }
    max = if current > max { current } else { max };
    max
}


fn solve_part2(contents: &String) -> i32 {
    let split = contents.split("\r\n");
    let mut current: i32 = 0;
    let mut calories = Vec::new();
    for s in split {
        if s == "" {
            calories.push(current);
            current = 0;
        } else {
            current += s.parse::<i32>().expect("Not a number!");
        }
    }
    calories.push(current);
    calories.sort();
    calories.reverse();
    calories[0] + calories[1] + calories[2]  
}
