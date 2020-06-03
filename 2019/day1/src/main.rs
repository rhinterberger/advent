use std::fs;

fn main() {
    let modules = fs::read_to_string("input.txt")
        .expect("Cannot open [input.txt]")
        .lines()
        .map(parse_mass)
        .collect::<Vec<i32>>();

    let fuel = modules
        .iter()
        .fold(0, |sum, mass| sum + fuel_simple(*mass));

    println!("Part 1 : Total Fuel needed {}", fuel);

    let fuel = modules
        .iter()
        .fold(0, |sum, mass| sum + fuel_total(*mass));

    println!("Part 2 : Total Fuel needed {}", fuel);
}

fn parse_mass(int_text: &str) -> i32 {
    match int_text.parse::<i32>() {
        Ok(mass) => mass,
        Err(error) => {
            println!("Parse Error {} : [{}]", error, int_text);
            0
        }
    }
}

fn fuel_total(module:i32) -> i32 {
    let mut sum :i32 = 0;

    let mut fuel_needed = fuel_simple(module);
    while fuel_needed > 0 {
        sum += fuel_needed;
        fuel_needed = fuel_simple(fuel_needed);
    }
    sum
}

fn fuel_simple(mass: i32) -> i32 {
    mass/3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mass() {
        assert_eq!(parse_mass("1"), 1);
        assert_eq!(parse_mass("9999"), 9999);
        assert_eq!(parse_mass("100.0"), 0);
        assert_eq!(parse_mass("10000000000"), 0);
        assert_eq!(parse_mass("I'm not parsable"), 0);
    }

    #[test]
    fn test_fuel_formula() {
        assert_eq!(fuel_simple(12), 2);
        assert_eq!(fuel_simple(14), 2);
        assert_eq!(fuel_simple(1969), 654);
        assert_eq!(fuel_simple(100756), 33583);
    }

    #[test]
    fn test_total_fuel_per_module() {
        assert_eq!(fuel_total(14), 2);
        assert_eq!(fuel_total(1969), 966);
        assert_eq!(fuel_total(100756), 50346);
    }
}