use std::fs;

fn main() {

    let input = read_input("input.txt");

    let product :i32 = find_sum_2020(&input, 2)
        .unwrap()
        .into_iter()
        .product();

    println!("{}",product);
}

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(parse_input_line)
        .collect::<Vec<i32>>()
}

fn parse_input_line(int_text: &str) -> i32 {
    match int_text.parse::<i32>() {
        Ok(number) => number,
        Err(error) => {
            println!("Parse Error {} : [{}]", error, int_text);
            0
        }
    }
}
/* FIXME */
fn find_sum_2020(numbers: &Vec<i32>, summands: i32) -> Option<Vec<i32>> {
    for i in 0..numbers.len() {
        for j in i+1 .. numbers.len() {
            for k in j+1 .. numbers.len() {
                if is2020(&vec!(numbers[i], numbers[j], numbers[k])) {
                    return Some(vec!(numbers[i], numbers[j], numbers[k]));
                }
            }
        }
    }
    None
}

fn is2020(summands: &Vec<i32>) -> bool {
    summands.into_iter().sum::<i32>() == 2020
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_2020() {
        let numbers  =vec!(2000,20);
        assert_eq!(is2020(&numbers),true);
    }

    #[test]
    fn not_2020() {
        let numbers  =vec!(2000,50);
        assert_eq!(is2020(&numbers),false);
    }

    #[test]
    fn find_sum_is_2020() {
        let numbers = vec!(2,3,4,5,6,7,20,2000);
        assert_eq!(find_sum_2020(numbers,2), Some(vec!(20, 2000)));

        let numbers = vec!(2,3,4,5,6,7,10,2010);
        assert_eq!(find_sum_2020(numbers,2), Some(vec!(10, 2010)));

        let numbers=vec!(1721,979,366,299,675,1456);
        assert_eq!(find_sum_2020(numbers,2), Some(vec!(1721, 299)));

        let numbers=vec!(1721,979,366,299,675,1456);
        assert_eq!(find_sum_2020(numbers,3), Some(vec!(979, 366, 675)));
    }

    #[test]
    #[should_panic]
    fn read_non_existing() {
        read_input("non_existing_file.txt");
    }

    #[test]
    fn read_input_txt() {
        let input_data=read_input("input.txt");
        assert!(input_data.len() > 0);
    }
/*
    #[test]
    fn run_over_slice() {
        let numbers = vec!(1,2,3,4,5,6,7,8,9,10);

    }
*/
}