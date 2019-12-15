use std::{fs, thread};
use itertools::{Itertools};
use std::sync::mpsc::{channel, Sender, Receiver};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let code = filecontents.split(",").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<i32>>();
