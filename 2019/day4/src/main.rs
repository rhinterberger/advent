fn main() {
    let input_range = 353096..843212;

    let mut count = 0;
    let mut count_pairs = 0;

    for key in input_range {
        let digits = split_to_digits(key);

        if ascending(&digits)
        {
            if check_repeat(&digits) { count +=1; }
            if check_pairs(&digits) { count_pairs +=1; }
        }
    }
    println!("{} Keys found", count);
    println!("{} Keys with pairs found", count_pairs);
}

fn check_pairs(digits: &[i32;6]) -> bool {
    let mut group_len = 1;
    for i in 0..5 {
        if digits[i] == digits[i+1] {
            group_len +=1;
        }
        else {
            if group_len == 2 {
                return true
            }
            group_len = 1;
        }
    }
    group_len == 2
}

fn check_repeat(digits: &[i32;6]) -> bool {
    for i in 0..5 {
        if digits[i] == digits[i+1] {
            return true;
        }
    }
    false
}

fn ascending(digits : &[i32; 6]) -> bool {
    for i in 0..5 {
        if digits[i] > digits[i+1] {
            return false;
        }
    }
    true
}

fn split_to_digits(mut num: i32) -> [i32; 6]
{
    let mut digits :[i32; 6] = [0; 6];
    let mut digit_index = 6;

    while num > 0 {
        digit_index -= 1;
        digits[digit_index] = num % 10;
        num /= 10;
    }
    digits
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_split_to_digits() {
        assert_eq!(split_to_digits(123456), [1,2,3,4,5,6]);
    }
    #[test]
    fn test_ascending() {
        assert_eq!(ascending(&[1,2,3,4,5,6]), true);
        assert_eq!(ascending(&[1,6,3,4,5,6]), false);
    }
    #[test]
    fn test_check_repeat() {
        assert_eq!(check_repeat(&[1,1,3,4,5,6]), true);
        assert_eq!(check_repeat(&[1,2,3,4,5,6]), false);
    }
    #[test]
    fn test_check_pairs() {
        assert_eq!(check_pairs(&[1,1,3,4,5,6]), true);
        assert_eq!(check_pairs(&[1,2,3,4,5,6]), false);
    }
}