use std::fs;
use std::iter::zip;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    zip(
        get_data(&contents).iter().skip(19).step_by(40),
        (20..=220).step_by(40),
    )
    .map(|(x, y)| x * y)
    .sum()
}

fn solve_part2(contents: &String) -> String {
    print_data(get_data(&contents));
    String::from("Look up")
}

fn get_data(contents: &String) -> Vec<i32> {
    let mut data: Vec<i32> = vec![];
    let mut val = 1;
    for line in contents.lines() {
        match line {
            "noop" => data.push(val),
            _ => {
                data.push(val);
                data.push(val);
                val += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            }
        }
    }
    data
}

fn print_data(data: Vec<i32>) {
    zip(0.., data.iter())
        .map(|(cycle, x)| {
            if (cycle % 40 - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<char>>()
        .chunks(40)
        .for_each(|s| println!("{}", s.iter().collect::<String>()));
}
