fn main()
{
    let mut count = 0;
    let mut count_pairs = 0;

    for key in 353096..843212 {
        let digits = split_to_digits(key);

        if ascending(digits)
        {
            if check_repeat(digits) { count +=1; }
            if check_pairs(digits) { count_pairs +=1; println!("{}",key); }
        }
    }
    println!("{} Keys found", count);
    println!("{} Keys with pairs found", count_pairs);
}

fn check_pairs(digits: [i32;6]) -> bool {
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

fn check_repeat(digits: [i32;6]) -> bool {
    for i in 0..5 {
        if digits[i] == digits[i+1] {
            return true;
        }
    }
    false
}

fn ascending(digits : [i32; 6]) -> bool {
    for i in 0..5 {
        if digits[i] < digits[i+1] {
            return false;
        }
    }
    true
}

fn split_to_digits(mut num: i32) -> [i32; 6]
{
    let mut digits :[i32; 6] = [0; 6];
    let mut digit_index = 0;

    while num > 0 {
        digits[digit_index] = num % 10;
        num /= 10; digit_index +=1;
    }

    digits
}