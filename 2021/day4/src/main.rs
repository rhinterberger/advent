use std::fs;

fn main() {

    let bingodata =fs::read_to_string("input.txt")
        .expect(&format!("Cannot open []"));

    let mut lines = bingodata
        .lines()
        .collect::<Vec<&str>>()
        .into_iter();

    let drawnumbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();

    let mut board= Board::new();
    for line in lines {
        if line.is_empty() {
            if !board.is_empty() {
                boards.push(board);
                board=Board::new();
            }
        }
        else {
            board.add_line(line);
        }
    }

    let mut winning_order:Vec<usize> = Vec::new();

    for number in drawnumbers {
        for i in 0..boards.len() {
            if !winning_order.contains(&i) {
                boards[i].mark_number(number);

                if boards[i].is_bingo() {
                        winning_order.push(i);
                }
            }
        }
    }

    let sum = boards[winning_order[0]].get_points();
    println!("Part 1: {}",sum);

    let sum = boards[winning_order[winning_order.len() -1]].get_points();
    println!("Part 2: {}",sum);
}

struct Board {
    lines: Vec<Vec<i32>>,
    last_number: i32
}

impl Board {
    fn new() -> Board {
        Board {lines: Vec::new(), last_number: -1}
    }

    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    fn add_line(&mut self, line: &str) {
        self.lines.push(
            line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        );
    }

    fn is_bingo(&self) -> bool {
        for x in 0..5 {
            let mut rowsum = 0;
            let mut colsum = 0;
            for y in 0..5 {
                if self.lines[x][y] == -1 {
                    rowsum += 1;
                }
                if self.lines[y][x] == -1 {
                    colsum += 1;
                }
                if rowsum == 5 || colsum == 5 {
                    return true;
                }
            }
        }
        false
    }

    fn mark_number(&mut self, value: i32) {
        for row in 0..self.lines.len() {
            for col in 0..self.lines[row].len() {
                if value.eq(&self.lines[row][col]) {
                    self.lines[row][col] = -1;
                    self.last_number = value;
                }
            }
        }
    }

    fn get_points(&self) -> i32 {
        self.last_number * self.lines
            .iter()
            .fold(0,|acc:i32, row|
                acc + row
                    .iter()
                    .map(|value|
                        match value {
                            -1 => 0,
                            _ => *value
                        }
                    )
                    .sum::<i32>()
            )
    }
}