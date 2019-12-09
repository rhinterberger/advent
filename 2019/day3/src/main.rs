use std::fs;
use std::cmp::{min, max};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut paths = filecontents.lines();

    let path1 = paths.next().unwrap();
    let path2 = paths.next().unwrap();

    let mut p1_lines = generate_lines(path1);
    let mut p2_lines = generate_lines(path2);

    let intersections = intersect_lines(&mut p1_lines,&mut p2_lines);
    let min_distance = intersections.clone().into_iter().min_by_key(|intersection| intersection.point.manhattan()).unwrap().point.manhattan();
    println!("Minimum Manthattan Distance: {}", min_distance);

    let min_delay = intersections.clone().into_iter().min_by_key(|intersection| intersection.get_delay()).unwrap().get_delay();
    println!("Minimum Delay : {}", min_delay);
}

fn generate_lines(path_str : &str) -> Vec<Line>
{
    let path = path_str.split(",");
    let mut lines :Vec<Line> = Vec::new();

    let mut current_line = Line {start: Point {x: 0, y:0}, end: Point {x:0,y:0}, steps: 0};

    for delta in path {
        current_line.start=current_line.end;

        let (direction, distance) = delta.split_at(1);
        let d = distance.parse::<i32>().unwrap();

        match direction {
            "R" => current_line.end.x += d,
            "L" => current_line.end.x -= d,
            "U" => current_line.end.y += d,
            "D" => current_line.end.y -= d,
            _ => panic!(),
        }

        current_line.steps += d;
        lines.push(current_line);
    }

    return lines;
}

fn intersect_lines(l1: &mut Vec<Line>, l2: &mut Vec<Line>) -> Vec<Intersection>
{
    let mut intersections :Vec<Intersection> = Vec::new();
    for line1 in l1.to_vec() {
        for line2 in l2.to_vec() {
            match line1.intersect(&line2) {
                Some(point) => {intersections.push(Intersection{ line1, line2, point})},
                None => {},
            }
        }
    }
    intersections
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x:i32,
    y:i32
}

impl Point
{
    fn manhattan(&self) -> i32
    {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    steps: i32
}

impl Line
{

    fn intersect(&self, line: &Line) -> Option<Point>
    {
        let mut p :Point = Point {x:0, y:0};
        // Horizontal
        if  min(self.start.x, self.end.x) <= line.start.x &&
            max(self.start.x, self.end.x) >= line.start.x &&
            min(line.start.y, line.end.y) <= self.start.y &&
            max(line.start.y, line.end.y) >= self.start.y
        {
            p = Point {x: line.start.x, y: self.start.y};
        }
        // Vertical
        if  min(self.start.y, self.end.y) <= line.start.y &&
            max(self.start.y, self.end.y) >= line.start.y &&
            min(line.start.x, line.end.x) <= self.start.x &&
            max(line.start.x, line.end.x) >= self.start.x
        {
            p = Point {x: self.start.x, y: line.start.y};
        }

        if p.x != 0 || p.y != 0{
            return Some(Point {x: line.start.x, y: self.start.y});
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
    line1: Line,
    line2: Line,
    point: Point
}

impl Intersection
{
    fn get_delay(&self) -> i32
    {
        let delta_l1 = self.line1.end.x.abs() - self.point.x.abs() + self.line1.end.y.abs() - self.point.y.abs();
        let delta_l2 = self.line2.end.x.abs() - self.point.x.abs() + self.line2.end.y.abs() - self.point.y.abs();

        self.line1.steps + self.line2.steps - delta_l1.abs() - delta_l2.abs()
    }
}