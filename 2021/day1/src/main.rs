use std::fs;

fn main() {

    let input = read_input("input.txt");

    let count :i32 = count_depth_increase(&input.clone());
    println!("{} increases",count);

    let count :i32 = count_sliding_window_increase(&input.clone());
    println!("{} increases",count);

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

fn count_depth_increase(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i-1] < depths[i] {
            count+=1;
        }
    }
    count
}

fn count_sliding_window_increase(depths: &Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..depths.len()-3 {
        if sliding_window_sum(depths,i) < sliding_window_sum(depths,i+1) {
            count+=1;
        }
    }
    count
}

fn sliding_window_sum(depths: &Vec<i32>, start: usize) -> i32 {
    depths[start] + depths[start+1] + depths[start+2]
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_count_depth_increases() {
        let data = vec![199,200, 208,210,200,207,240,269,260,263];

        assert_eq!(count_depth_increase(&data), 7);
    }

    #[test]
    fn test_sliding_window() {
        let data = vec![199,200, 208,210,200,207,240,269,260,263];

        assert_eq!(sliding_window_sum(&data, 0), 607);
        assert_eq!(sliding_window_sum(&data, 1), 618);
        assert_eq!(sliding_window_sum(&data, 2), 618);
        assert_eq!(sliding_window_sum(&data, 3), 617);
        assert_eq!(sliding_window_sum(&data, 4), 647);
        assert_eq!(sliding_window_sum(&data, 5), 716);
        assert_eq!(sliding_window_sum(&data, 6), 769);
        assert_eq!(sliding_window_sum(&data, 7), 792);
    }

    #[test]
    fn test_sliding_window_max() {
        let data = vec![199,200, 208,210,200,207,240,269,260,263];

        assert_eq!(count_sliding_window_increase(&data), 5);
    }

}