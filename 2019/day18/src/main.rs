use std::{fs, thread};
use std::sync::mpsc::{channel, Sender, Receiver};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();
}