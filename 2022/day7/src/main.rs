use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs;

fn main() {
    let input_string = fs::read_to_string("input2.txt").expect("Failed to open [input.txt]");

    let dirs = prepare_tree(input_string);

    let sum = dirs.borrow()
        .into_iter()
        .filter(|(_path, size)| **size <= 100000)
        .fold(0,|sum, (_path, size)| sum+size);

    println!("{:?}", sum);

    let need_to_delete = 30000000 - (70000000 - dirs.get("/").unwrap());
    let dir = dirs
        .into_iter()
        .filter(|(_path, size)| *size >= need_to_delete)
        .min_by(|a,b| a.1.cmp(&b.1))
        .unwrap();

    println!("{}", dir.1);
}

fn prepare_tree(input_string: String) -> HashMap<String, i32> {
    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut cwd = String::new();

    for line in input_string
        .lines()
        .filter(ignore_lines)
    {
        let line_result = parse_line(line, &cwd);
        cwd = line_result.0;

        let mut parent_path = cwd.to_string();
        while parent_path != "/" {
            parent_path = parent_path
                .rsplit_once("#")
                .unwrap()
                .0
                .to_string();

            dirs.entry(parent_path.to_string())
                .and_modify(|v| { *v += line_result.1 })
                .or_insert(line_result.1);
        }
    }
    dirs
}

fn ignore_lines(line: &&str) -> bool {
    *line != "$ ls" && ! (*line).starts_with("dir ")
}

fn parse_line(line: &str, cwd: &String)  -> (String, i32){
    let mut tokens = line.rsplitn(2," ");
    let target = tokens.next().unwrap();
    let command = tokens.next().unwrap();

    let mut new_path = cwd.to_string();
    let mut size = 0;
    match command {
        "$ cd" => {
            if target == ".." {
                new_path.pop();
                new_path = new_path.rsplit_once("#").unwrap().0.to_string();
            }
            else {
                new_path.push_str(target);
            }
            new_path.push_str("#");
        },
        _ => { size = command.parse::<i32>().unwrap(); }
    };

    (new_path, size)
}