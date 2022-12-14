use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

fn solve_part1(contents: &String) -> usize {
    solve(contents, 2)
}

fn solve_part2(contents: &String) -> usize {
    solve(contents, 10)
}

fn solve(contents: &String, knot_count: usize) -> usize {
    let moves = Regex::new(r"([UDLR]) (\d+)").unwrap();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); knot_count];
    for line in contents.lines() {
        let m = moves.captures(line).unwrap();
        let delta = match &m[1] {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, 1),
            "U" => (0, -1),
            _ => panic!("invalid input"),
        };
        for _ in 0..m[2].parse::<i32>().unwrap() {
            knots[knot_count - 1] = (
                knots[knot_count - 1].0 + delta.0,
                knots[knot_count - 1].1 + delta.1,
            );
            for n in (0..knot_count - 1).rev() {
                knots[n] = update_tail(knots[n], knots[n + 1]);
            }
            seen.insert(knots[0]);
        }
    }
    seen.len()
}

fn update_tail((tail_x, tail_y): (i32, i32), (head_x, head_y): (i32, i32)) -> (i32, i32) {
    let dx = head_x - tail_x;
    let dy = head_y - tail_y;
    if dx.abs() + dy.abs() == 3 {
        if dx.abs() == 2 {
            (tail_x + dx / 2, tail_y + dy)
        } else {
            (tail_x + dx, tail_y + dy / 2)
        }
    } else {
        (tail_x + dx / 2, tail_y + dy / 2)
    }
}

fn _print_knots(arr: &[(i32, i32)]) {
    println!("[");
    for (a, b) in arr {
        print!("({},{})", a, b);
    }
    print!("]");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up() {
        let result = update_tail((0, 0), (0, 2));
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn down() {
        let result = update_tail((0, 2), (0, 0));
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn right() {
        let result = update_tail((0, 0), (2, 0));
        assert_eq!(result, (1, 0));
    }

    #[test]
    fn left() {
        let result = update_tail((0, 0), (-2, 0));
        assert_eq!(result, (-1, 0));
    }

    #[test]
    fn diag_stay() {
        let result = update_tail((0, 0), (1, 1));
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn diag_ne() {
        let result = update_tail((0, 0), (1, 2));
        assert_eq!(result, (1, 1));
    }

    #[test]
    fn diag_se() {
        let result = update_tail((0, 0), (-1, 2));
        assert_eq!(result, (-1, 1));
    }
}
