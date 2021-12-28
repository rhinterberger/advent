use std::fs;

fn main() {

    let mut hm = HeightMap::new(read_input("input.txt"));
    println!("{}",hm.get_risk());

    let mut basins = hm.low_points
        .clone()
        .map(|low_point|
            hm.fill_basin(&low_point)
        )
        .collect::<Vec<usize>>();

    basins.sort_by(|a, b| b.cmp(a));
    let result: usize = basins.iter().take(3).product();
    println!("{:?}", result);
}

fn read_input(path: &str) -> Vec<Vec<i32>> {

    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(|line|
            line.chars()
                .map( |height| height.to_string().parse::<i32>().unwrap() )
                .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>()
}

struct HeightMap {
    data: Vec<Vec<i32>>,
    low_points: LowPoints
}
impl HeightMap {

    fn new(heightmap: Vec<Vec<i32>>) -> HeightMap {
        let mut hm = HeightMap {data: heightmap, low_points: LowPoints {data: Vec::new(), pos: 0} };
        hm.find_low_points();
        hm
    }

    fn find_low_points(&mut self) {
        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                let point = Point {x, y, value: self.data[y][x]};
                let neighbors = self.get_neighbors(&point);

                let lowest_neighbor = neighbors
                    .iter()
                    .min_by(|a,b|
                        a.value.cmp(&b.value)
                    )
                    .unwrap();

                if point.value < lowest_neighbor.value {
                    self.low_points.data.push(point);
                }
            }
        }
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        if point.x != 0 {
            neighbors.push(Point{ x: point.x-1, y: point.y, value: self.data[point.y][point.x - 1]});
        }
        if point.y != 0 {
            neighbors.push(Point{ x: point.x, y: point.y-1, value: self.data[point.y-1][point.x]});
        }
        if point.x != self.data[0].len() - 1 {
            neighbors.push(Point{ x: point.x+1, y: point.y, value: self.data[point.y][point.x+1]});
        }
        if point.y != self.data.len() - 1 {
            neighbors.push(Point{ x: point.x, y: point.y+1, value: self.data[point.y+1][point.x]});
        }

        neighbors
    }

    fn get_risk(&mut self) -> i32 {
        self.low_points.clone().fold(0, |sum, point| sum+point.value+1)
    }

    fn fill_basin(&self, basin: &Point) -> usize {
        let mut search_stack: Vec<Point> = Vec::new();
        let mut basin_elements: Vec<Point> = Vec::new();

        search_stack.push(basin.clone());
        basin_elements.push(basin.clone());
        while !search_stack.is_empty() {
            let current_point = search_stack.pop().unwrap();
            let neigh = self.get_neighbors(&current_point);
            for neighbor in neigh {
                if neighbor.value < 9 {
                    if !basin_elements.contains(&neighbor) {
                        search_stack.push(neighbor.clone());
                        basin_elements.push(neighbor.clone());
                    }
                }
            }
        }
        basin_elements.len()
    }
}

#[derive(Debug, Clone)]
struct LowPoints {
    data: Vec<Point>,
    pos: usize
}

impl Iterator for LowPoints {
    type Item<> = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if self.pos-1 == self.data.len() {
            return None;
        }

        Some(self.data[self.pos-1].clone())
    }
}

#[derive(Debug,Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    value: i32
}