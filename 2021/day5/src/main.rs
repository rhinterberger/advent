use std::cmp::max;
use std::fs;

const X_SIZE:usize = 1000;
const Y_SIZE:usize = 1000;

fn main() {
    let lines = read_input("input.txt");

    let mut board = vec![0; X_SIZE*Y_SIZE];
    for line in &lines {
        if line.is_vertical() || line.is_horizontal() {
            draw(&mut board, line);
        }
    }
    let sum = board.into_iter().filter(|value| *value >= 2).count();
    println!("{}", sum);

    let mut board = vec![0; X_SIZE*Y_SIZE];
    for line in &lines {
            draw(&mut board, line);
    }
    let sum = board.into_iter().filter(|value| *value >= 2).count();
    println!("{}", sum);

}

fn read_input(path: &str) -> Vec<Line> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(|line| parse_lines(line))
        .collect::<Vec<Line>>()
}

fn parse_lines(line: &str) -> Line {
    let coordinates =  line
        .replace(" -> ",",")
        .split(",")
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Line::new(coordinates)
}

fn draw(board: &mut Vec<i32>, line: &Line) {
    let length = max(line.length.x, line.length.y);
    for i in 0..length+1 {
        let x = line.start.x + i*line.delta.x;
        let y = line.start.y + i*line.delta.y;
        board[x as usize + y as usize * Y_SIZE] += 1;
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    delta: Point,
    length: Point
}
impl Line {

    fn new(coords: Vec<i32>) -> Line {
        let start = Point {x: coords[0], y: coords[1]};
        let end = Point {x: coords[2], y: coords[3]};

        let dx = (end.x - start.x).signum();
        let dy = (end.y - start.y).signum();

        let lx = (end.x - start.x).abs();
        let ly = (end.y - start.y).abs();

        let delta = Point {x: dx, y:dy};
        let length = Point {x: lx, y:ly};

        Line { start, end, delta, length }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}
