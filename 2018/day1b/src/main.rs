use std::env;
use std::process;
use std::fs;
use std::collections::HashMap;

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

    println!("Found first duplicate: {}", find_sum(filecontents));
}

fn find_sum(filecontents: String) -> i32 
{
    let mut sum = 0;
    let mut sums = HashMap::new();

    loop
    {
        let numbers = filecontents.lines().into_iter().map(|valstr| valstr.parse::<i32>());
    
        for num in numbers
        {
            sum += num.unwrap();
            if sums.contains_key(&sum)    
            {
                return sum;         
            }
            else
            {
                sums.insert(sum, true);
            }
        }
    }
}