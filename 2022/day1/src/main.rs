use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");

    let top_three_elves = parse_input(input_string);
    let sum_top_three :i32 = top_three_elves[0] + top_three_elves[1] + top_three_elves[2];

    println!("Part 1: {}", top_three_elves[0]);
    println!("Part 2: {}", sum_top_three);
}

fn parse_input(input_string: String) -> Vec<i32> {
    let mut top_three: Vec<i32> = vec!(0; 3);
    let mut calories = 0;

    for line in input_string.lines() {
        match line {
            "" => {  sort_top_three(&mut top_three, calories); calories = 0; }
            _ => calories += line.parse::<i32>().unwrap()
        }
    }
    top_three
}

fn sort_top_three(top_three: &mut Vec<i32>, calories: i32) {
    if calories > top_three[2] {
        top_three[2] = calories;
    }

    if top_three[2] > top_three[1] {
        top_three.swap(1, 2)
    }

    if top_three[1] > top_three[0] {
        top_three.swap(0, 1)
    }
}