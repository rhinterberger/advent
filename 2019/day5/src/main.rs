mod intcode;

use std::{fs};
use intcode::{Intcode};

fn main()
{
    let program = fs::read_to_string("input.txt")
        .expect("Cannot open [input.txt]")
        .split(",")
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut computer = Intcode::new(program.clone());
    computer.run(1);

    let mut computer = Intcode::new(program.clone());
    computer.run(5);
}