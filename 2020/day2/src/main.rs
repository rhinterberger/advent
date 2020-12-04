use std::fs;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Policy {
    character: char,
    min: i32,
    max: i32
}

fn main() {
    println!("{}", read_input("input.txt"));
    println!("{}", read_input2("input.txt"));
}

fn read_input(path: &str) -> i32 {
    let mut number_valid_passwords = fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .fold(0, |valid_passwords, line| valid_passwords + isValidPassword(line));

    number_valid_passwords
}

fn isValidPassword(line: &str) ->i32 {
    let mut line_parts = line.split(' ');
    let minmax = line_parts.next().unwrap();
    let character = line_parts.next().unwrap();
    let password = line_parts.next().unwrap();

    let policy: Policy = get_minmax(minmax);
    let character = get_character(character);

    let valid_chars = password
        .chars()
        .fold(0, |num_char, c| {
            if c == character
            {
                return num_char+1;
            }
            num_char
        });

    if policy.min <= valid_chars && valid_chars <= policy.max {
        return 1;
    }
    0
}

fn read_input2(path: &str) -> i32 {
    let mut number_valid_passwords = fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .fold(0, |valid_passwords, line| valid_passwords + isValidPassword2(line));

    number_valid_passwords
}

fn isValidPassword2(line: &str) ->i32 {
    let mut line_parts = line.split(' ');
    let minmax = line_parts.next().unwrap();
    let character = line_parts.next().unwrap();
    let password:Vec<char> = line_parts.next().unwrap().chars().collect();

    let policy: Policy = get_minmax(minmax);
    let character = get_character(character);

    if  (password[policy.min as usize -1] == character ||
        password[policy.max as usize -1] == character ) &&
        password[policy.min as usize -1] != password[policy.max as usize -1] {
        return 1;
    }
    0
}

fn get_minmax(minmax: &str) -> Policy {
     let mut mm= minmax
         .split("-")
         .map(|num| num.parse::<i32>().unwrap() );

    let min = mm.next().unwrap();
    let max = mm.next().unwrap();
    Policy { character: ' ', min, max }
}

fn get_character(input: &str) -> char {
    input.chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn read_non_existing() {
        read_input("non_existing_file.txt");
    }

/*    #[test]
    fn read_input_txt() {
        let input_data=read_input("input.txt");
        assert!(input_data.len() > 0);
    }
*/
    #[test]
    fn parse_line() {
        let line = "1-3 a: abcde";
        assert_eq!(isValidPassword(line),1);
        let line = "1-3 b: cdefg";
        assert_eq!(isValidPassword(line),0);
    }

    #[test]
    fn parse_min_max()
    {
        let minmax="1-3";
        assert_eq!(get_minmax(minmax), Policy {character: ' ',min: 1,max: 3});
    }

    #[test]
    fn parse_character()
    {
        let character="a:";
        assert_eq!(get_character(character), 'a');
    }
}