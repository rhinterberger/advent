use std::fs;
use std::cmp::{min, max};

fn main() {
    let wires = fs::read_to_string("input.txt")
        .expect("Cannot open file [input.txt]")
        .lines()
        .map(|line| generate_wire_paths(&line.to_string()))
        .collect::<Vec<Vec<PathSegment>>>();

    let intersections = find_intersections(&wires);

    let min_distance = intersections
        .iter()
        .min_by_key(|intersection| intersection.point.manhattan())
        .unwrap()
        .point.manhattan();
    println!("Part 1: Minimum Manhattan Distance: {}", min_distance);

    let min_delay = intersections
        .iter()
        .min_by_key(|intersection| intersection.get_delay())
        .unwrap()
        .get_delay();
    println!("Part 2: Minimum Delay : {}", min_delay);
}

fn generate_wire_paths(path_str: &String) -> Vec<PathSegment> {
    let mut prev_segment = PathSegment::new();

    path_str
        .split(",")
        .map(|segment_text| {
            prev_segment = generate_path_segment(segment_text, prev_segment);
            prev_segment
        })
        .collect::<Vec<PathSegment>>()
}

fn generate_path_segment(segment_text: &str, previous_segment: PathSegment) -> PathSegment {

    let (direction, distance) = convert_segment_text(segment_text);

    let mut new_segment = PathSegment {
        start: previous_segment.end,
        end: previous_segment.end,
        length: previous_segment.length + distance
    };

    match direction {
        "R" => new_segment.end.x += distance,
        "L" => new_segment.end.x -= distance,
        "U" => new_segment.end.y += distance,
        "D" => new_segment.end.y -= distance,
        _ => panic!(),
    }

    new_segment
}

fn convert_segment_text(segment_text: &str) -> (&str, i32) {
    let (direction, distance) = segment_text.split_at(1);

    ( direction, distance.parse::<i32>().unwrap() )
}

fn find_intersections(wires: &Vec<Vec<PathSegment>>) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = Vec::new();

    for line1 in &wires[0] {
        for line2 in &wires[1] {
            match line1.intersect(line2) {
                Some(point) => {
                    intersections.push( Intersection { line1: line1.clone(), line2: line2.clone(), point } )
                },
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

impl Point {
    fn new() -> Point {
        Point {x:0, y:0}
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct PathSegment {
    start: Point,
    end: Point,
    length: i32
}

impl PathSegment
{
    fn new() -> PathSegment {
        PathSegment { start : Point::new(), end : Point::new(), length: 0}
    }

    fn intersect(&self, test_segment: &PathSegment) -> Option<Point> {
        let mut intersection_point = None;

        if !self.is_parallel(test_segment) {
            if self.is_horizontal() {
                intersection_point = self.get_intersection_point(test_segment);
            } else {
                intersection_point = test_segment.get_intersection_point(self);
            }
        }

        intersection_point
    }

    fn get_intersection_point(&self, test_segment: &PathSegment) -> Option<Point> {

        if  min(self.start.x, self.end.x) <= test_segment.start.x &&
            max(self.start.x, self.end.x) >= test_segment.start.x &&
            min(test_segment.start.y, test_segment.end.y) <= self.start.y &&
            max(test_segment.start.y, test_segment.end.y) >= self.start.y
        {
            return Some( Point { x:test_segment.start.x, y:self.start.y});
        }

        None
    }

    fn is_parallel(&self, test_segment: &PathSegment) -> bool {
        (self.start.x == self.end.x && test_segment.start.x == test_segment.end.x) ||
        (self.start.y == self.end.y && test_segment.start.y == test_segment.end.y)
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Intersection {
    line1: PathSegment,
    line2: PathSegment,
    point: Point
}

impl Intersection
{
    fn get_delay(&self) -> i32 {
        let delta_l1 = (self.line1.end.manhattan() - self.point.manhattan()).abs();
        let delta_l2 = (self.line2.end.manhattan() - self.point.manhattan()).abs();

        self.line1.length + self.line2.length - delta_l1 - delta_l2
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_manhattan() {
        let point = Point {x:0, y:0};
        assert_eq!(point.manhattan(),0);
        let point = Point {x:1, y:0};
        assert_eq!(point.manhattan(),1);
        let point = Point {x:0, y:1};
        assert_eq!(point.manhattan(),1);
        let point = Point {x:50, y:50};
        assert_eq!(point.manhattan(),100);
        let point = Point {x:-50, y:50};
        assert_eq!(point.manhattan(),100);
        let point = Point {x:50, y:-50};
        assert_eq!(point.manhattan(),100);
        let point = Point {x:-50, y:-50};
        assert_eq!(point.manhattan(),100);
    }

    #[test]
    fn test_horizontal() {
        let segment = PathSegment { start: Point { x: 0, y: 0 }, end: Point { x: 10, y: 0 }, length: 0 };
        assert_eq!(segment.is_horizontal(), true);

        let segment = PathSegment { start: Point {x:0, y:0}, end: Point {x:0, y:10}, length: 0 };
        assert_eq!(segment.is_horizontal(), false);
    }

    #[test]
    fn test_parallel() {
        // Horizontal Parallel
        let segment_a = PathSegment { start: Point {x:0, y:0}, end: Point {x:10, y:0}, length: 0 };
        let segment_b = PathSegment { start: Point {x:0, y:1}, end: Point {x:10, y:1}, length: 0 };
        assert_eq!(segment_a.is_parallel(&segment_b), true);
        assert_eq!(segment_b.is_parallel(&segment_a), true);

        // Vertical Parallel
        let segment_a = PathSegment { start: Point {x:0, y:0}, end: Point {x:0, y:10}, length: 0 };
        let segment_b = PathSegment { start: Point {x:1, y:0}, end: Point {x:1, y:10}, length: 0 };
        let segment_b = PathSegment { start: Point {x:1, y:0}, end: Point {x:1, y:10}, length: 0 };
        let segment_b = PathSegment { start: Point {x:1, y:0}, end: Point {x:1, y:10}, length: 0 };
        assert_eq!(segment_a.is_parallel(&segment_b), true);
        assert_eq!(segment_b.is_parallel(&segment_a), true);

        // intersecting
        let segment_a = PathSegment { start: Point {x:0, y:0}, end: Point {x:10, y:0}, length: 0 };
        let segment_b = PathSegment { start: Point {x:5, y:-5}, end: Point {x:5, y:5}, length: 0 };
        assert_eq!(segment_a.is_parallel(&segment_b), false);
        assert_eq!(segment_b.is_parallel(&segment_a), false);
    }

    #[test]
    fn test_intersect() {
        // Horizontal Parallel
        let segment_a = PathSegment { start: Point {x:0, y:0}, end: Point {x:10, y:0}, length: 0 };
        let segment_b = PathSegment { start: Point {x:0, y:1}, end: Point {x:10, y:1}, length: 0 };
        assert_eq!(segment_a.intersect(&segment_b).is_some(), false);
        assert_eq!(segment_b.intersect(&segment_a).is_some(), false);

        // intersecting
        let segment_a = PathSegment { start: Point {x:0, y:0}, end: Point {x:10, y:0}, length: 0 };
        let segment_b = PathSegment { start: Point {x:5, y:-5}, end: Point {x:5, y:5}, length: 0 };
        assert_eq!(segment_a.intersect(&segment_b).is_some(), true);
        assert_eq!(segment_b.intersect(&segment_a).is_some(), true);
    }
}