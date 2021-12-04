use std::fs;

const VALUELEN: usize = 12;

fn main() {
    let diagnostics = read_input("input.txt");

    let most_common = find_most_common(&diagnostics);
    let gamma_rate = calculate_gamma(&most_common);
    let epsilon_rate = invert(gamma_rate);
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Power Consumption: {}",power_consumption);

    let oxygen_rating = filter_values(diagnostics.clone(),VALUELEN, false);
    let co2_rating    = filter_values(diagnostics.clone(),VALUELEN, true);
    println!("Life Support Rating: {}", oxygen_rating as i32 * co2_rating as i32);
}

fn read_input(filename: &str) -> Vec<u16> {
    fs::read_to_string(filename)
        .expect(&format!("Cannot open [{}]", filename))
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u16>>()
}

fn filter_values(diagnostic_codes: Vec<u16>, maskpos: usize, notone: bool) -> u16 {
    if diagnostic_codes.len() == 1 {
        return diagnostic_codes[0];
    }
    let most_common = find_most_common(&diagnostic_codes);
    let most_common_bit = notone ^ is_one_most_common(maskpos, &most_common);
    let filtered = diagnostic_codes
        .into_iter()
        .filter(|value| filter_value(value, maskpos-1, most_common_bit))
        .collect::<Vec<u16>>();

    filter_values(filtered.to_vec(), maskpos-1, notone)
}

fn is_one_most_common(maskpos: usize, most_common: &Vec<i32>) -> bool {
    match most_common[VALUELEN - maskpos] {
        1 => true,
        _ => false
    }
}

fn filter_value(value: &u16, maskpos: usize, common_one:bool) -> bool {
    let mask = 1 << maskpos;
    if value&mask != 0 {
        return common_one;
    }
    !common_one
}

fn find_most_common(diagnostics: &Vec<u16>) -> Vec<i32> {
    let mut ones = vec![0; VALUELEN];

    count_ones(diagnostics, &mut ones);
    calc_most_common(diagnostics.len() as i32, &mut ones);

    ones
}

fn count_ones(diagnostics: &Vec<u16>, ones: &mut Vec<i32>) {
    for line in diagnostics {
        let mut mask = 1 << VALUELEN-1;
        for i in 0..VALUELEN as usize {
            if line & mask != 0 {
                ones[i] += 1;
            }
            mask >>= 1;
        }
    }
}

fn calc_most_common(len: i32, ones: &mut Vec<i32>) {

    for i in 0..VALUELEN as usize {
        if (len - ones[i]) <= ones[i] {
            ones[i] = 1;
        } else {
            ones[i] = 0;
        }
    }
}

fn calculate_gamma(ones: &Vec<i32>) -> i32 {
    let mut gamma_rate = 0;

    for i in 0..VALUELEN as usize {
        if ones[i] == 1 {
            gamma_rate += 1 << (VALUELEN-1-i);
        }
    }
    gamma_rate
}

fn invert(value: i32) -> i32 {
    match VALUELEN {
         5 => value^0b11111,
        12 => value^0b111111111111,
         _ => value
    }
}