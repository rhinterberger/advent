use std::fs;
use std::ops::{Add, Index};
use itertools::Itertools;

fn main() {
    let mut ledisplay = &read_input("input.txt");

    let mut count = 0;

    for line in ledisplay {
        count += line.output.iter()
            .fold(0, |acc, segments|
                match segments.len() {
                    2 | 3 | 4 | 7 => acc + 1,
                    _ => acc
                }
            )
    }

    println!("Count: {}", count);

    let mut sum_of_all = 0;
    let permutations = "abcdefg".chars().permutations(7);
    for perm in permutations {
        for line in ledisplay{
            let mut count = 0;
            for signal in &line.input {
                let match_found = find_match(&perm, signal);
                if match_found == 0 {
                    break;
                }
                else {
                    count += match_found;
                }
            }
            if count == 10 {
                sum_of_all += get_output_value(&perm, &line.output);
            }
        }
    }
    println!("Sum: {}", sum_of_all);

}

fn get_output_value(perm: &Vec<char>, output: &Vec<String>) -> i32 {

    let symbols = vec![
        0b1110111,  // 0
        0b0010010,  // 1
        0b1011101,  // 2
        0b1011011,  // 3
        0b0111010,  // 4
        0b1101011,  // 5
        0b1101111,  // 6
        0b1010010,  // 7
        0b1111111,  // 8
        0b1111011   // 9
    ];

    let mut value = 0;
    for digit in output {
        let number = digit.chars()
            .fold(0,|mut acc, c|
                acc+2_i32.pow(6 - (perm.iter().position(|p| *p == c).unwrap() as u32))
            );

        value = value * 10 + symbols.iter().position(|p| *p == number).unwrap();
    }
    value as i32
}


fn find_match(perm: &Vec<char>, signal: &String) -> i32 {

    let led_positions = vec![
        vec![0, 1, 2, 4, 5, 6],  // 0
        vec![2, 5],    // 1
        vec![0, 2, 3, 4, 6],  // 2
        vec![0, 2, 3, 5, 6],  // 3
        vec![1, 2, 3, 5],    // 4
        vec![0, 1, 3, 5, 6],  // 5
        vec![0, 1, 3, 4, 5, 6],  // 6
        vec![0, 2, 5],    // 7
        vec![0, 1, 2, 3, 4, 5, 6],   // 8
        vec![0, 1, 2, 3,    5, 6]   // 9
    ];

    match signal.len() {
        2 => check_leds(&perm, signal, &led_positions[1]),
        3 => check_leds(&perm, signal, &led_positions[7]),
        4 => check_leds(&perm, signal, &led_positions[4]),
        5 => check_leds(&perm, signal, &led_positions[2]) +
             check_leds(&perm, signal, &led_positions[3]) +
             check_leds(&perm, signal, &led_positions[5]),
        6 => check_leds(&perm, signal, &led_positions[0]) +
             check_leds(&perm, signal, &led_positions[6]) +
             check_leds(&perm, signal, &led_positions[9]),
        7 | _ => check_leds(&perm, signal, &led_positions[8])
    }
}

fn check_leds(perm: &Vec<char>, signal: &String, symbol: &Vec<i32>) -> i32 {
    let mut selected_chars = String::new();
    for led in symbol {
        selected_chars.push(perm[*led as usize]);
    }

    let mut count = 0;
    if selected_chars.chars().sorted().collect::<String>().eq(signal) {
        count = 1;
    }

    count
}

struct DisplaySignal {
    input: Vec<String>,
    output: Vec<String>,
}

fn read_input(path: &str) -> Vec<DisplaySignal> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let input  = prepare_input(parts.next().unwrap().trim().to_string());
            let output = prepare_output(parts.next().unwrap().trim().to_string());
            DisplaySignal {input, output }
        })
        .collect::<Vec<DisplaySignal>>()
}

fn prepare_input(input: String) -> Vec<String> {

    input.split(' ')
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()) )
        .map(|signal|
            signal.chars().sorted().collect::<String>()
        )
        .collect::<Vec<String>>()
}

fn prepare_output(output: String) -> Vec<String> {

    output.split(' ')
        .map(|signal|
            signal.chars().sorted().collect::<String>()
        )
        .collect::<Vec<String>>()
}