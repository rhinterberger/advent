use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");

    let mut elves = parse_input(input_string);

    elves.sort_unstable();
    println!("Part 1: {}", elves.last().unwrap());

    let sum_top_three :i32 = elves
        .into_iter()
        .rev()
        .take(3)
        .sum();
    println!("Part 2: {}", sum_top_three);
}

fn parse_input(input_string: String) -> Vec<i32> {
    let mut elves: Vec<i32> = Vec::new();
    let mut calories=0;

    for line in input_string.lines() {
        match line {
            "" => {
                elves.push(calories);
                calories = 0;
            }
            _ => calories += line.parse::<i32>().unwrap()
        }
    }
    elves
}
