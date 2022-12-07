use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string(".\\input.txt")
    .expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> i32 {
    contents.split("\r\n")
    .map(|sack| {
        let mid = sack.len() / 2;
        (sack.chars().take(mid).collect(), sack.chars().skip(mid).take(mid).collect())
    })
    .map(|(compartment1, compartment2)| find_same(compartment1, compartment2))
    .map(|item| get_priority(item.chars().next().unwrap()))
    .sum()
}

fn find_same(s1: String, s2: String) -> String {
    let mut seen = HashSet::new();
    s1.chars().for_each(|c| { seen.insert(c); });
    s2.chars().filter(|c| seen.contains(c)).collect()
}

fn get_priority(item: char) -> i32 {
    match item {
        item if item.is_uppercase() => item as i32 - 38,
        item if item.is_lowercase() => item as i32 - 96,
        _ => panic!(),
    }
}

fn solve_part2(contents: &String) -> i32 {
    let mut split = contents.split("\r\n").peekable();
    let mut ans = 0;
    while split.peek().is_some() {
        let word1 = split.next().unwrap().to_string();
        let word2 = split.next().unwrap().to_string();
        let word3 = split.next().unwrap().to_string();
        ans += get_priority(find_same(find_same(word1, word2),word3).chars().next().unwrap());
    }
    ans
}
