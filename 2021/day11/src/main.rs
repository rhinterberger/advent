use std::fs;

const FLASH_THRESHOLD: i32 = 9;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");

    // Part 1
    let mut cavern = parse_dumbo_array(&input_string);
    cavern.iterate(100);
    println!("Number of flashes after 100 iterations: {}", cavern.flash_count);

    // Part 2
    cavern = parse_dumbo_array(&input_string);
    let iteration = cavern.iterate(i32::MAX-1).unwrap();
    println!("All Dumbos flashed at iteration: {}", iteration);
}

fn parse_dumbo_array(input: &String) -> DumboCavern {
    let cavern_data = input.lines()
        .map(|line|
            line.chars()
                .map( |energy| energy.to_string().parse::<i32>().unwrap() )
                .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();

    DumboCavern{ cavern: cavern_data.clone(), size_x: cavern_data[0].len(), size_y: cavern_data.len(), flash_threshold: FLASH_THRESHOLD, flash_count: 0 }
}

struct DumboCavern {
    cavern: Vec<Vec<i32>>,
    size_x: usize,
    size_y: usize,
    flash_threshold: i32,
    flash_count: i32,
}

impl DumboCavern {
    fn iterate(self: &mut Self, num: i32) -> Option<i32>  {
        for iteration_num in 1..num+1 {
            self.increment();
            self.flash_all();
            self.reset_flashed_dumbos();
            if self.check_all_flashed()  {
                return Some(iteration_num);
            }
        }
        None
    }

    fn increment(self: &mut Self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                self.cavern[y][x] += 1;
            }
        }
    }

    fn flash_all(self: &mut Self)  {
        loop {
            let mut flashed = false;

            for y in 0..self.size_y {
                for x in 0..self.size_x {
                    if self.is_flashable(self.cavern[y][x]) {
                        flashed = true;
                        self.flash_count += 1;
                        self.cavern[y][x] = i32::MIN;

                        self.do_flash( x as i32, y as i32);
                    }
                }
            }
            if flashed == false {
                break;
            }
        }
    }

    fn reset_flashed_dumbos(self: &mut Self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.cavern[y][x] < 0 {
                    self.cavern[y][x] = 0;
                }
            }
        }
    }

    fn check_all_flashed(self: &Self) -> bool {
        let mut flashed_dumbos = 0;
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.cavern[y][x] == 0 {
                    flashed_dumbos += 1;
                }
            }
        }

        flashed_dumbos == self.size_x * self.size_y
    }

    fn is_flashable(self: &Self, dumbo: i32) -> bool {
        dumbo > self.flash_threshold
    }

    fn do_flash(self: &mut Self, x: i32, y: i32) {
        for dy in -1..2 {
            for dx in -1..2 {
                if self.is_within_bounds(x + dx, y + dy) {
                    self.cavern[(y + dy) as usize][(x + dx) as usize] += 1;
                }
            }
        }
    }

    fn is_within_bounds(self: &Self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.size_x as i32 &&
        y >= 0 && y < self.size_y as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "11111
19991
19191
19991
11111";

    const RESULT_SMALL_1: &str ="34543
40004
50005
40004
34543";

    const RESULT_SMALL_2: &str = "45654
51115
61116
51115
45654";

    #[test]
    fn should_return_result_small_1() {
        let mut input = parse_dumbo_array(&INPUT_SMALL.to_string());
        let expected_output = parse_dumbo_array(&RESULT_SMALL_1.to_string());

        input.iterate(1);
        assert_eq!(input.cavern, expected_output.cavern);
    }

    #[test]
    fn should_return_result_small_2() {
        let mut input = parse_dumbo_array(&INPUT_SMALL.to_string());
        let expected_output = parse_dumbo_array(&RESULT_SMALL_2.to_string());

        input.iterate(2);
        assert_eq!(input.cavern, expected_output.cavern);
    }

    const INPUT_BIG: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn should_return_1656_flashes() {
        let mut input = parse_dumbo_array(&INPUT_BIG.to_string());
        input.iterate(100);
        assert_eq!(input.flash_count, 1656);
    }

    #[test]
    fn should_stop_at_iteration_195() {
        let mut input = parse_dumbo_array(&INPUT_BIG.to_string());
        assert_eq!(input.iterate(i32::MAX-1), Some(195));
    }
}