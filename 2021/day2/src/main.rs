use std::fs;
use std::str::Lines;

fn main() {

    let position = fs::read_to_string("input.txt")
        .expect(&format!("Cannot open [input.txt]"))
        .lines()
        .map(|line| parse_position_line(line))
        .fold((0,0), | acc, x| (acc.0+x.0, acc.1+x.1));

    println!("{} ",position.0 * position.1);

    let directions = fs::read_to_string("input.txt")
        .expect(&format!("Cannot open [input.txt]"))
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut distance=0;
    let mut depth=0;
    let mut aim = 0;

    for line in directions {
        let direction: Vec<&str> = line.split(' ').collect();

        match direction[0] {
            "forward" => { distance += direction[1].parse::<i32>().unwrap(); depth += aim * direction[1].parse::<i32>().unwrap()},
            "up" => {aim += -1 * direction[1].parse::<i32>().unwrap();},
            "down" => {aim += direction[1].parse::<i32>().unwrap();},
            _ => ()
        }
    }
    println!("{} ",distance * depth);

}

fn parse_position_line(line: &str) -> (i32, i32) {
    let direction: Vec<&str> = line.split(' ').collect();
    let mut distance=0;
    let mut depth=0;
    match direction[0] {
        "forward" => {distance = direction[1].parse::<i32>().unwrap();},
        "up" => {depth = -1 * direction[1].parse::<i32>().unwrap();},
        "down" => {depth = direction[1].parse::<i32>().unwrap();},
        _ => ()
    }

    (distance, depth)
}

