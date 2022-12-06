use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");

    println!("Signal-Marker: {}", find_marker(&input_string, 4));
    println!("Message-Marker: {}", find_marker(&input_string, 14));
}

fn find_marker(signal_input: &String, match_len: usize) ->  usize{
    match_len + signal_input
        .as_bytes()
        .windows(match_len)
        .position(is_different_characters).unwrap()
}

fn is_different_characters(candidate: &[u8]) -> bool {
    let mut match_total = true;
    for i in 0..candidate.len() - 1 {
        for j in i + 1..candidate.len() {
            match_total &= candidate[i] != candidate[j];
        }
    }
    match_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_7() {
        assert_eq!(find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),4), 7)
    }
    // mjqjpqmgbljsphdztnvjfqwrcgsmlb  7

    #[test]
    fn should_return_5() {
        assert_eq!(find_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),4), 5)
    }
    // bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5

    #[test]
    fn should_return_6() {
        assert_eq!(find_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(),4), 6)
    }
    // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6

    #[test]
    fn should_return_10() {
        assert_eq!(find_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),4), 10)
    }
    // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10

    #[test]
    fn should_return_11() {
        assert_eq!(find_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),4), 11)
    }
    // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

    #[test]
    fn should_return_19() {
        assert_eq!(find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),14), 19)
    }

    #[test]
    fn should_return_23() {
        assert_eq!(find_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),14), 23)
    }

    #[test]
    fn should_return_23_2() {
        assert_eq!(find_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(),14), 23)
    }

    #[test]
    fn should_return_29() {
        assert_eq!(find_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),14), 29)
    }

    #[test]
    fn should_return_26() {
        assert_eq!(find_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),14), 26)
    }
}