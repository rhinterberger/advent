use std::env;
use std::process;
use std::fs;
use std::str;
use std::cmp::Ordering;

use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You have to provide an inputfile");
        process::exit(1);
    }

    let filename = &args[1];
    let mut filecontents = fs::read_to_string(filename).unwrap();

    let mut prev_char:char;

    for current_char in filecontents.chars() {

        if !prev_char
        {
            prev_char = current_char;
            continue;
        }

        if prev_char.is_lowercase()
        {
            if prev_char.to_uppercase() == current_char
            {
                // Annihilate !
            }
        }

        if prev_char.is_uppercase()
        {
            if prev_char.to_lowercase() == current_char
            {
                // Annihilate !
            }
        }


    }

}
