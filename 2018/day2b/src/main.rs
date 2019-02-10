use std::env;
use std::process;
use std::fs;
use std::str;

fn main()
{
    let args:Vec<String> = env::args().collect();

    if args.len() < 2
    {
        println!("You have to provide an Input-File");
        process::exit(1);       
    } 

    let filename = &args[1];
    let filecontents = fs::read_to_string(filename).unwrap();

    println!("Common Letters: {}", find_box_ids(filecontents));
}

fn compare_ids(s1: &str, s2: &str) -> String
{
    let mut common = String::from("");
    let mut mismatches = 0;

    let characters = s1.chars().zip(s2.chars());

    for char_tuple in characters
    {
        if char_tuple.0 != char_tuple.1 
        {
            mismatches += 1;
            if mismatches > 1 { return "".to_string(); }
            continue;
        }

        common.push(char_tuple.0);   
    }

    common.to_string()   
}

fn find_box_ids(set1: String) -> String
{
    let set2 = set1.clone();
    let mut common_chars: String;

    for line1 in set1.lines()
    {
        // set2 should be shortened to begin at pos of line1 + 1
        for line2 in set2.lines()
        {
            // could be removed if set2 is shortened
            if line1==line2
            {
                continue;
            }

            common_chars = compare_ids(line1, line2);   
            if !common_chars.is_empty()
            {
                return common_chars.to_string();
            }
        }
    }
    "NONE".to_string()
}