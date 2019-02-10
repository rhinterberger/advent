use std::env;
use std::process;
use std::fs;
use std::collections::HashMap;
use std::str;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        println!("You have to provide an Input-File");
        process::exit(1);       
    } 

    let filename = &args[1];
    let filecontents = fs::read_to_string(filename).unwrap();

    println!("Checksum: {}", aggregate(filecontents));
}

fn countletters(s: &str) -> (i32, i32)
{
    let mut charcounts = HashMap::new();
    let mut threes :i32 = 0;
    let mut twos :i32 = 0;
    
    for c in s.chars()
    {
        let count = charcounts.entry(c).or_insert(0);
        *count += 1;
    }

    for num in charcounts.values()
    {
        if *num == 3
        {
            threes=1; 
            continue;
        }
        if *num == 2
        {
            twos=1; 
            continue;
        }
    }
 
    (twos,threes)
}

fn aggregate(bigstring: String) -> i32
{
    let mut threes :i32 = 0;
    let mut twos :i32 = 0;
    
    for line in bigstring.lines()
    {
        let (two,three) = countletters(line);
        
        threes += three;
        twos += two;
    }
        
    (twos*threes)
}