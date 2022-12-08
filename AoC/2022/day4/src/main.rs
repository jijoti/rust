use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> usize {
    solve(contents, &are_ranges_contained)
}

fn are_ranges_contained(low1: i32, high1: i32, low2: i32, high2: i32) -> bool {
    (low1 <= low2 && high2 <= high1) || (low2 <= low1 && high1 <= high2)
}

fn solve_part2(contents: &String) -> usize {
    solve(contents, &do_ranges_overlap)
}

fn do_ranges_overlap(low1: i32, high1: i32, low2: i32, high2: i32) -> bool {
    (low1 <= high2) && (high1 >= low2)
}

// fn solve_old(contents: &String, f: &dyn Fn((i32, i32, i32, i32)) -> bool) -> usize {
//     contents
//         .lines()
//         .map(|elves| elves.split_once(',').unwrap())
//         .map(|(elf1, elf2)| (elf1.split_once('-').unwrap(), elf2.split_once('-').unwrap()))
//         .map(|((low1, high1), (low2, high2))| {
//             (
//                 low1.parse::<i32>().unwrap(),
//                 high1.parse::<i32>().unwrap(),
//                 low2.parse::<i32>().unwrap(),
//                 high2.parse::<i32>().unwrap(),
//             )
//         })
//         .filter(|&ranges| f(ranges))
//         .count()
// }

fn solve(contents: &String, f: &dyn Fn(i32, i32, i32, i32) -> bool) -> usize {
    contents
        .lines()
        .flat_map(|elves| elves.split(','))
        .flat_map(|elf| elf.split('-'))
        .map(|num_as_str| num_as_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(4)
        .filter(|nums| f(nums[0], nums[1], nums[2], nums[3]))
        .count()
}
