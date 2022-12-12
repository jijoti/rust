use regex::Regex;
use std::cell::RefCell;
use std::cmp::min;
use std::fs;
use std::rc::{Rc, Weak};

fn main() {
    let contents =
        fs::read_to_string(".\\input.txt").expect("Should have been able to read the file");
    println!("Answer to part 1: {}", solve_part1(&contents));
    println!("Answer to part 2: {}", solve_part2(&contents));
}

struct File {
    name: String,
    size: i32,
}

struct Directory {
    name: String,
    size: i32,
    files: Vec<File>,
    child_dirs: Vec<Rc<RefCell<Directory>>>,
    parent_dir: Option<Weak<RefCell<Directory>>>,
}

fn solve_part1(contents: &String) -> i32 {
    let dir: Rc<RefCell<Directory>> = build_dir(&contents);
    traverse_dir_and_add(&dir)
}

fn traverse_dir_and_add(dir: &Rc<RefCell<Directory>>) -> i32 {
    let mut ans = get_size_to_add(&dir);
    for child_dir in &dir.borrow().child_dirs {
        ans += traverse_dir_and_add(&child_dir);
    }
    ans
}

fn get_size_to_add(dir: &Rc<RefCell<Directory>>) -> i32 {
    let max_size = 100000;
    if dir.borrow().size <= max_size {
        dir.borrow().size
    } else {
        0
    }
}

fn solve_part2(contents: &String) -> i32 {
    let max_size = 40000000;
    let dir: Rc<RefCell<Directory>> = build_dir(&contents);
    let need_to_delete = dir.borrow().size - max_size;
    let ans = traverse_dir_and_find(&dir, need_to_delete, dir.borrow().size);
    ans
}

fn traverse_dir_and_find(dir: &Rc<RefCell<Directory>>, need_to_delete: i32, min_size: i32) -> i32 {
    let mut ans = find_size_to_delete(&dir, need_to_delete, min_size);
    for child_dir in &dir.borrow().child_dirs {
        ans = min(ans, traverse_dir_and_find(&child_dir, need_to_delete, ans));
    }
    ans
}

fn find_size_to_delete(dir: &Rc<RefCell<Directory>>, need_to_delete: i32, min_size: i32) -> i32 {
    if dir.borrow().size >= need_to_delete {
        min(dir.borrow().size, min_size)
    } else {
        min_size
    }
}

fn init_dir(root_name: String) -> Rc<RefCell<Directory>> {
    Rc::new(RefCell::new(Directory {
        name: root_name,
        size: 0,
        files: Vec::new(),
        child_dirs: Vec::new(),
        parent_dir: None,
    }))
}

fn add_dir(parent_dir: &Rc<RefCell<Directory>>, child_dir_name: String) {
    parent_dir
        .borrow_mut()
        .child_dirs
        .push(Rc::new(RefCell::new(Directory {
            name: child_dir_name,
            size: 0,
            files: Vec::new(),
            child_dirs: Vec::new(),
            parent_dir: Some(Rc::downgrade(parent_dir)),
        })));
}

fn add_file(parent_dir: &Rc<RefCell<Directory>>, file_name: String, file_size: i32) {
    parent_dir.borrow_mut().files.push(File {
        name: file_name,
        size: file_size,
    });
    update_dir_sizes(&Some(Rc::downgrade(parent_dir)), file_size);
}

fn update_dir_sizes(parent_dir: &Option<Weak<RefCell<Directory>>>, file_size: i32) {
    match parent_dir {
        Some(parent_dir) => {
            parent_dir.upgrade().unwrap().borrow_mut().size += file_size;
            update_dir_sizes(
                &parent_dir.upgrade().unwrap().borrow().parent_dir,
                file_size,
            );
        }
        None => {}
    }
}

fn find_child_dir(dir: &Rc<RefCell<Directory>>, child_dir_name: String) -> Rc<RefCell<Directory>> {
    Rc::clone(
        dir.borrow()
            .child_dirs
            .iter()
            .find(|child_dir| child_dir.borrow().name == child_dir_name)
            .unwrap(),
    )
}

fn get_parent_dir(dir: &Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
    dir.borrow().parent_dir.as_ref().unwrap().upgrade().unwrap()
}

fn build_dir(contents: &String) -> Rc<RefCell<Directory>> {
    let root_dir: Rc<RefCell<Directory>> = init_dir(String::from("/"));
    let mut current_dir: Rc<RefCell<Directory>> = Rc::clone(&root_dir);

    let change_to_child_dir = Regex::new(r"\$ cd ([a-z]+)").unwrap();
    let list_dir = Regex::new(r"dir ([a-z]+)").unwrap();
    let list_file = Regex::new(r"(\d+) ([a-z.]+)").unwrap();
    for line in contents.lines() {
        match line {
            "$ cd /" => current_dir = Rc::clone(&root_dir),
            "$ cd .." => current_dir = get_parent_dir(&current_dir),
            line if change_to_child_dir.is_match(line) => {
                current_dir = find_child_dir(
                    &current_dir,
                    change_to_child_dir.captures(line).unwrap()[1].to_string(),
                )
            }
            "$ ls" => {}
            line if list_dir.is_match(line) => {
                add_dir(
                    &current_dir,
                    list_dir.captures(line).unwrap()[1].to_string(),
                );
            }
            line if list_file.is_match(line) => {
                let caps = list_file.captures(line).unwrap();
                add_file(
                    &current_dir,
                    caps[2].to_string(),
                    caps[1].parse::<i32>().unwrap(),
                );
            }
            _ => panic!("Unexpected input {}", line),
        }
    }
    // print_dir(&root_dir);
    root_dir
}

// fn print_dir(dir: &Rc<RefCell<Directory>>) {
//     print_dir_internal(dir, "");
// }

// fn print_dir_internal(dir: &Rc<RefCell<Directory>>, tabs: &str) {
//     println!("{}/{} - {}", tabs, &dir.borrow().name, &dir.borrow().size);
//     let next_tabs = &(tabs.to_owned() + "    ");
//     for file in &dir.borrow().files {
//         println!("{}{} - {}", next_tabs, &file.name, &file.size);
//     }
//     for child_dir in &dir.borrow().child_dirs {
//         print_dir_internal(child_dir, next_tabs);
//     }
// }
