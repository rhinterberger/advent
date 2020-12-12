use std::fs;

fn main() {
    let numbers = read_input("input.txt");
    let no_pair = check_pairs(numbers,25);
    println!("{:?}", no_pair);

    let numbers = read_input("input.txt");
    let weakness_sum = find_weakness(numbers,no_pair.unwrap());
    println!("{:?}", weakness_sum);
}

fn read_input(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(parse_input_line)
        .collect::<Vec<i64>>()
}

fn parse_input_line(int_text: &str) -> i64 {
    match int_text.parse::<i64>() {
        Ok(number) => number,
        Err(error) => {
            println!("Parse Error {} : [{}]", error, int_text);
            0
        }
    }
}

fn check_pairs(numbers: Vec<i64>, preamble_length: usize) -> Option<i64> {
    for i in preamble_length..numbers.len() {
        let mut pair_found= false;
        for j in i-preamble_length .. numbers.len() {
            for k in 0..preamble_length {
                if numbers[j]+numbers[i-k] == numbers[i] {
                    pair_found = true;
                }
            }
        }
        if !pair_found {
            return Some(numbers[i]);
        }
    }
    None
}

fn find_weakness(numbers: Vec<i64>, weakness: i64) -> Option<i64> {
    let mut weak_sum: i64;
    for i in 0..numbers.len() {
        for j in i..numbers.len() {

            let current_slice = numbers[i..j].iter();
            weak_sum = current_slice.clone().sum();
            if weak_sum == weakness {
                return Some(current_slice.clone().min().unwrap() + current_slice.clone().max().unwrap());
            }

        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        let numbers = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let invalid = check_pairs(numbers, 5);
        assert_eq!(invalid, Some(127));
    }

    #[test]
    fn test_xmas_weakness() {
        let numbers = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let weakness = find_weakness(numbers, 127);
        assert_eq!(weakness, Some(62));
    }
}
