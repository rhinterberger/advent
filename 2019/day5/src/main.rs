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

    Intcode::new(program.clone(), 1)
        .run();

    Intcode::new(program.clone(),5)
        .run();
}