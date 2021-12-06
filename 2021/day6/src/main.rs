use std::fs;

fn main() {
    let mut fish = read_input("input.txt");

    let mut fish_state = count_fish(&mut fish);
    let sum = simulate(&mut fish_state, 80);
    println!("{}", sum);

    let mut fish_state = count_fish(&mut fish);
    let sum = simulate(&mut fish_state, 256);
    println!("{}", sum);
}

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .trim()
        .split(",")
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn count_fish(fish: &mut Vec<usize>) -> Vec<usize> {
    let mut fish_state: Vec<usize> = vec![0; 9];

    for state in 0..9 {
        fish_state[state] = fish
            .into_iter()
            .filter(|current_state| **current_state == state)
            .count() ;
    }

    fish_state
}

fn simulate(fish: &mut Vec<usize>, generations: usize) -> usize {

    for _generation in 0..generations {
        let fish_in_state_zero = fish[0];
        for state in 1..9 as usize {
            fish[state-1] = fish[state];
        }
        fish[6] += fish_in_state_zero;
        fish[8] = fish_in_state_zero;
    }

    fish.iter().sum()
}