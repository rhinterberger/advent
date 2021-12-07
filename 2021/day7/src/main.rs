use std::fs;

fn main() {
    let crabs = &read_input("input.txt");

    let min = calc_fuel(crabs, |targetpos:i32, pos:i32| (pos - targetpos).abs() );
    println!("Part 1: {:?}",min);

    let min = calc_fuel(crabs, |targetpos:i32, pos:i32| (0 .. 1+(targetpos - pos).abs()).sum() );
    println!("Part 2: {:?}",min);
}

fn calc_fuel<T>(crabs: &Vec<i32>, fuel_function: T) -> i32
    where T: Fn(i32, i32) -> i32
{
    let max = crabs.iter().max().unwrap();
    let mut sums = vec![0; *max as usize];

    for targetpos in 0.. *max {
        let fuel: Vec<i32> = crabs
            .iter()
            .map(|pos| fuel_function(targetpos, *pos) )
            .collect();

        sums[targetpos as usize] =  fuel.into_iter().sum::<i32>();
    }

    sums.iter().min().unwrap().clone()
}

fn read_input(path: &str) -> Vec<i32> {

    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .trim()
        .split(",")
        .map(|position| position.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}