use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");
    let priority_items = find_priority_items(rucksacks.as_str());
    println!("Sum of Priority Items: {}",calculate_priority(priority_items));

    let priority_items = find_badges(rucksacks.as_str());
    println!("Sum of Badge Items: {}",calculate_priority(priority_items));
}

fn find_priority_items(rucksacks: &str) -> Vec<u64> {
    let mut prio_items:Vec<u64> = Vec::new();
    rucksacks.lines().for_each( | rucksack| {
        let compartments = split_into_compartments(rucksack);
        let priority_of_current_rucksack = convert_to_alphabet_bitset(compartments.0) &
                                                convert_to_alphabet_bitset(compartments.1);
        prio_items.push(priority_of_current_rucksack);
    });
    prio_items
}

fn find_badges(rucksacks: &str) -> Vec<u64> {
    let mut badge_priorities:Vec<u64> = Vec::new();
    rucksacks.lines().for_each( | rucksack| {
        let items_bits = convert_to_alphabet_bitset(rucksack.to_string());
        badge_priorities.push(items_bits);
    });

    let mut badge_items:Vec<u64> = Vec::new();
    for elve_group in badge_priorities.chunks(3) {
        let bagde = elve_group[0] & elve_group[1] & elve_group[2];
        badge_items.push(bagde);
    }
    badge_items
}

fn calculate_priority(priority_items: Vec<u64>) -> i32 {
    let mut sum_of_priorities = 0;
    for items in priority_items {
        sum_of_priorities += get_bit_positions(items)
    }
    sum_of_priorities
}

fn get_bit_positions(items: u64) -> i32 {
    let mut sum_of_bit_positions = 0;
    for i in 0..64 {
        if items & (1 << i) != 0 {
            sum_of_bit_positions += i + 1;
        }
    }
    sum_of_bit_positions
}

fn split_into_compartments(rucksack: &str) -> (String, String) {
    let compartments = rucksack.split_at(rucksack.len()/2);
    (compartments.0.to_string(), compartments.1.to_string())
}

fn convert_to_alphabet_bitset(rucksack_items: String) -> u64 {
    rucksack_items.chars().fold(0, |alphabet, current_item| {
        let bit_position = match current_item.is_ascii_lowercase() {
            true => current_item as u64 - 97,        // Lowercase letters start at ASCII 97
            false => current_item as u64 - 65 + 26   // Uppercase letters start at ASCII 65, but need Offset for Priorities
        };
        alphabet | (1 << bit_position)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_bits_for_items() {
        assert_eq!(convert_to_alphabet_bitset("a".to_string()), 1 << 0);
        assert_eq!(convert_to_alphabet_bitset("A".to_string()), 1 << 26);
        assert_eq!(convert_to_alphabet_bitset("abc".to_string()), 0b0000000000000000000000000000000000000000000000000000000000000111);
        assert_eq!(convert_to_alphabet_bitset("ABC".to_string()), 1 << 26 | 1 << 27 | 1 << 28);
    }

    #[test]
    fn should_split_rucksack_compartments() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(split_into_compartments(rucksack), ("vJrwpWtwJgWr".to_string(),"hcsFMMfFFhFp".to_string()));
    }

    #[test]
    fn should_find_piority_item_p() {
        let rucksacks = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(find_priority_items(rucksacks).pop().unwrap(), 1<<15);
    }

    #[test]
    fn should_find_piority_item_uppercase_l() {
        let rucksacks = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(find_priority_items(rucksacks).pop().unwrap(), 1<<37);
    }

    #[test]
    fn should_return_priority_sum_157() {
        let rucksacks = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let priority_items = find_priority_items(rucksacks);
        assert_eq!(calculate_priority(priority_items),157);
    }

}