use std::fs;
use std::process::exit;

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut phase1 = filecontents.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();

    // Part 1
    let phase2 = vec![0; phase1.len()];
    let mut phases = [phase1.clone(), phase2];

    let base_pattern = [0, 1, 0, -1];
    let mut num_phases = 100;
    let mut current_input = 0;
    let mut current_output = 1;

    while num_phases > 0 {
        for j in 0..phases[current_output].len() {
            phases[current_output][j] = 0;
            for i in 0..phases[current_input].len() {
                let base_index = ((i as i32 + 1)/ (j as i32 + 1)) %4;
                phases[current_output][j] += phases[current_input][i] * base_pattern[base_index as usize];
            }
            phases[current_output][j] = phases[current_output][j].abs() % 10;
        }

        current_input = (current_input + 1)%2;
        current_output = (current_output + 1)%2;

        num_phases -=1;
    }

    for i in 0..8 {
        println!("{} : {}", i, phases[current_input][i] );
    }

    // Part2

    phases[0] = phase1.clone();
    let factor = 10000;
    for i in 0..factor-1 {
        phases[0].append(&mut phase1.clone());
    }

    let mut index = 0;
    for i in 0..7 {
        index += phase1[6 - i] * 10i32.pow(i as u32);
    }

    num_phases = 100;
    let len =  phases[0].len() - 1;
    while num_phases > 0 {
        for j in 0..phases[0].len() -1 - index as usize {
            let ind = len - j;
            phases[0][ind  -1 as usize] = (phases[0][ind] + phases[0][ind-1 as usize]) % 10;
        }
        num_phases -=1;
    }

    for i in index as usize..index as usize + 8 as usize {
        println!("{} : {}", i, phases[0][i] );
    }
}
