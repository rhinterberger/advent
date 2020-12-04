use std::fs;

fn main() {
    let landscape = read_input("input.txt");
    let trees = count_trees(&landscape,3,1);
    println!("Trees: {}", trees);

    let mut trees = count_trees(&landscape, 1,1);
    trees *= count_trees(&landscape, 3,1);
    trees *= count_trees(&landscape, 5,1);
    trees *= count_trees(&landscape, 7,1);
    trees *= count_trees(&landscape, 1,2);

    println!("Tree Multiply: {}", trees);
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(parse_landscape)
        .collect::<Vec<Vec<char>>>()
}

fn parse_landscape(int_text: &str) -> Vec<char> {
    let mut landscape_line:Vec<char> = Vec::new();
    int_text
        .chars()
        .for_each(|element| {
            landscape_line.push(element);
        });

    landscape_line
}

fn count_trees(landscape: &Vec<Vec<char>>, slope_x: usize, slope_y: usize) -> i32 {
    let mut count = 0;
    let mut position = Point {x: 0, y:0, slope_x, slope_y};
    while position.y < landscape.len()-1 {
        if position.slope_move().is_tree(&landscape) {
            count += 1;
        }
    }
    count
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    slope_x: usize,
    slope_y: usize
}

impl Point {
    pub fn slope_move(self: &mut Self) -> &Self{
        self.x += self.slope_x;
        self.y += self.slope_y;
        self
    }

    pub fn is_tree(self: &Self, landscape: &Vec<Vec<char>>) -> bool {
        let width = landscape[0].len();
        landscape[self.y][self.x % width] == '#'
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_landscape() -> Vec<Vec<char>> {
        vec!(
            vec!('.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'),
            vec!('#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'),
            vec!('.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'),
            vec!('.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'),
            vec!('.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'),
            vec!('.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'),
            vec!('.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'),
            vec!('.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'),
            vec!('#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'),
            vec!('#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'),
            vec!('.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#')
        )
    }

    #[test]
    fn three_left_one_down() {
        let mut position = Point {x:0, y:0, slope_x: 3, slope_y: 1};
        position.slope_move();
        assert_eq!(position, Point {x: 3, y: 1, slope_x: 3, slope_y: 1});
    }

    #[test]
    fn check_tree() {
        let position = Point {x: 3, y:0, slope_x: 3, slope_y: 1};
        let landscape = create_landscape();
        assert_eq!(position.is_tree(&landscape), true)
    }
    #[test]
    fn check_no_tree() {
        let position = Point {x: 0, y:0, slope_x: 3, slope_y: 1};
        let landscape = create_landscape();
        assert_eq!(position.is_tree(&landscape), false)
    }

    #[test]
    fn move_and_test() {
        let mut position = Point {x: 0, y:0, slope_x: 3, slope_y: 1};
        let landscape = create_landscape();
        assert_eq!(position.slope_move().is_tree(&landscape), false)
    }

    #[test]
    fn repeat_pattern_and_test() {
        let mut position = Point {x: 4, y:1, slope_x: 3, slope_y: 1};
        let landscape = create_landscape();
        assert_eq!(position.slope_move().is_tree(&landscape), false)
    }

    #[test]
    fn count_all_trees_3_1() {
        let landscape = create_landscape();
        assert_eq!(count_trees(&landscape, 3, 1), 7)
    }

    #[test]
    fn count_all_trees_1_1() {
        let landscape = create_landscape();
        assert_eq!(count_trees(&landscape, 1, 1), 2)
    }

    #[test]
    fn count_all_trees_5_1() {
        let landscape = create_landscape();
        assert_eq!(count_trees(&landscape, 5, 1), 3)
    }

    #[test]
    fn count_all_trees_7_1() {
        let landscape = create_landscape();
        assert_eq!(count_trees(&landscape, 7, 1), 4)
    }

    #[test]
    fn count_all_trees_1_2() {
        let landscape = create_landscape();
        assert_eq!(count_trees(&landscape, 1, 2), 2)
    }

    #[test]
    #[should_panic]
    fn input_not_readable() {
        read_input("non_existing_file.txt");
    }

    #[test]
    fn landscape_line() {
        assert_eq!(parse_landscape("..##......."),
                   vec!('.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'));
    }
}