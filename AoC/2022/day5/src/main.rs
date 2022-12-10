use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> String {
    solve(contents, &move_crates_one)
}

fn solve_part2(contents: &String) -> String {
    solve(contents, &move_crates_multi)
}

fn solve(
    contents: &String,
    move_crates: &dyn Fn(&mut HashMap<char, Vec<char>>, char, char, i32),
) -> String {
    let mut lines = contents.lines();
    let mut stacks = build_initial_stacks(&mut lines);
    // skip empty line
    lines.next().unwrap();
    for line in lines {
        let (from, to, amount) = get_move(line);
        move_crates(&mut stacks, from, to, amount);
    }
    let mut keys = stacks.keys().collect::<Vec<&char>>();
    keys.sort();
    keys.iter()
        .map(|key| stacks.get(key).unwrap().last().unwrap())
        .collect()
}

fn get_move(line: &str) -> (char, char, i32) {
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    let cap = re.captures(line).unwrap();
    (
        cap[2].parse::<char>().unwrap(),
        cap[3].parse::<char>().unwrap(),
        cap[1].parse::<i32>().unwrap(),
    )
}

fn move_crates_one(stacks: &mut HashMap<char, Vec<char>>, from: char, to: char, amount: i32) {
    // Cannot borrow two objects mutably from HashMap.
    // let from_stack: &mut Vec<char> = stacks.get_mut(&from).unwrap();
    // let to_stack: &mut Vec<char> = stacks.get_mut(&to).unwrap();
    for _n in 1..=amount {
        let item = { stacks.get_mut(&from).unwrap().pop().unwrap() };
        stacks.get_mut(&to).unwrap().push(item);
    }
}

fn move_crates_multi(stacks: &mut HashMap<char, Vec<char>>, from: char, to: char, amount: i32) {
    let items = {
        let crates = stacks.get_mut(&from).unwrap();
        &mut crates.split_off(crates.len() - amount as usize)
    };
    stacks.get_mut(&to).unwrap().append(items);
}

fn build_initial_stacks(lines: &mut std::str::Lines) -> HashMap<char, Vec<char>> {
    let mut stack_input = Vec::<&str>::new();
    let stack_metadata = loop {
        let line = lines.next().unwrap();
        if line.contains("[") {
            stack_input.push(line);
        } else {
            break get_stack_metadata(line);
        }
    };
    populate_stacks(&mut stack_input, stack_metadata)
}

fn get_stack_metadata(stack_input: &str) -> Vec<(char, usize)> {
    let mut stack_metadata = Vec::new();
    let mut pos: usize = 0;
    for c in stack_input.chars() {
        if !c.is_whitespace() {
            stack_metadata.push((c, pos));
        }
        pos += 1;
    }
    stack_metadata
}

fn populate_stacks(
    stack_input: &mut Vec<&str>,
    stack_metadata: Vec<(char, usize)>,
) -> HashMap<char, Vec<char>> {
    let mut stacks = initialize_stacks(&stack_metadata);
    while let Some(level) = stack_input.pop() {
        for (id, pos) in &stack_metadata {
            let put_on_stack = level.chars().nth(*pos).unwrap();
            if !put_on_stack.is_whitespace() {
                stacks.get_mut(id).unwrap().push(put_on_stack);
            }
        }
    }
    stacks
}

fn initialize_stacks(stack_metadata: &Vec<(char, usize)>) -> HashMap<char, Vec<char>> {
    let mut stacks = HashMap::new();
    for (id, _) in stack_metadata {
        stacks.insert(*id, Vec::new());
    }
    stacks
}

// fn print_stacks(stacks: &HashMap<char, Vec<char>>) {
//     for (key, val) in stacks.iter() {
//         print!("\nkey: {} val:", key);
//         print_stack(val);
//     }
// }

// fn print_stack(stack: &Vec<char>) {
//     for c in stack {
//         print!("{}", c);
//     }
// }
