use std::io;
use std::io::prelude::*;

fn main()
{
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines()
    {
        sum += line.unwrap().parse::<i32>().unwrap();        
    }
    
    println!("{}",sum);
}