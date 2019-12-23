use std::{fs, thread};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut repairbot: Vec<i64> = filecontents.trim().split(",").map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    repairbot.resize(repairbot.len() + 100_000, 0);

    let (tobot, fromparent) = channel();
    let (toparent, frombot) = channel();

    thread::spawn(move || { run(fromparent, toparent, &mut repairbot); });

    let directions = [
        Point {x:  0, y: -1},  // north
        Point {x:  0, y:  1},  // south
        Point {x: -1, y:  0},  // west
        Point {x:  1, y:  0},  // east
    ];

    let mut direction = 0usize;
    let mut position = Point { x: 0, y: 0 };
    let start_position = position;
    let mut dest_position = Point {x:0 ,y:0};
    let mut found = false;

    let mut maze = HashMap::new();
    maze.entry(position).or_insert(4);

    loop {
        tobot.send(direction as i32 +1 ).unwrap();
        let status:i32 = match frombot.recv() { Ok(status) => status, _ => break };

        // Simple maze mapping
        match status {
                0 => { // if wall, turn right
                    maze.entry(position.add(directions[direction])).or_insert( status);
                    position.sub(directions[direction]);
                    match direction {  // turn right
                        0 => direction = direction + 2,
                        1 => direction = direction + 2,
                        2 => direction = direction - 1,
                        3 => direction = direction - 3,
                        _ => println!("invalid direction {}", direction)
                    };
                },
                1 => { // if no wall, turn and try left
                    position = position.add(directions[direction]);
                    maze.entry(position).or_insert( status);
                    match direction {  // turn left
                        0 => direction = direction + 3,
                        1 => direction = direction + 1,
                        2 => direction = direction - 2,
                        3 => direction = direction - 2,
                        _ => println!("invalid direction {}", direction)
                    };

                }
                2 => { //found oxygen turn left
                    position = position.add(directions[direction]);
                    maze.entry(position).or_insert( status);
                    match direction {  // turn left
                        0 => direction = direction + 3,
                        1 => direction = direction + 1,
                        2 => direction = direction - 2,
                        3 => direction = direction - 2,
                        _ => println!("invalid direction {}", direction)
                    };
                    found = true;
                    dest_position = position;

                },
                _ => { println!("Invalid status {}", status); break;} // Invalid
        };

        if found && position == start_position { direction = 98; }
    }

    let shortest_path = find_shortest_path(maze.clone(), start_position, dest_position);
    println!("{}",shortest_path);

    let minutes = fill(maze.clone(), dest_position);
    println!("{}",minutes);
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Node {
    point: Point,
    dist: i32,
}

fn find_shortest_path(maze: HashMap<Point, i32>, start: Point, dest: Point) -> i32
{
    let neighbors = [Point {x:1, y:0},Point {x:0, y:1},Point {x:-1, y:0},Point {x:0, y:-1}];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(Node {point: start, dist:0});

    while !queue.is_empty() {
        let  current = queue.pop_front().unwrap();

        if current.point == dest {
            return current.dist;
        }

        for i in 0..4 {
            let neighbor = current.point + neighbors[i];

            if !visited.contains(&neighbor) &&
                match maze.get(&neighbor) { Some(status) => *status != 0 , _ => false }
            {
                visited.insert(current.point);
                queue.push_back(Node {point: neighbor, dist: current.dist+1});
            }
        }
    }
    -1
}

fn fill(mut maze: HashMap<Point, i32>, start: Point) -> i32
{
    let neighbors = [Point {x:1, y:0},Point {x:0, y:1},Point {x:-1, y:0},Point {x:0, y:-1}];
    let mut points_to_fill :[VecDeque<Point>;2] = [VecDeque::new(), VecDeque::new()];
    let mut minutes = -1;
    let mut current_queue=0;

    points_to_fill[current_queue].push_back(start);

    loop {
       while !points_to_fill[current_queue].is_empty() {
            let current = points_to_fill[current_queue].pop_front().unwrap();

            maze.entry(current).and_modify(|state| *state = 2);

            for i in 0..4 {
                let neighbor = current + neighbors[i];

                if *maze.get(&neighbor).unwrap() == 1
                {
                    points_to_fill[(current_queue + 1) % 2].push_back(neighbor);
                }
            }
        }
        minutes += 1;
        current_queue = (current_queue + 1) %2;
        if points_to_fill[current_queue].is_empty() {break;}
    }
    minutes
}

pub fn run(input: Receiver<i32>, output: Sender<i32>, code: &mut Vec<i64>)
{
    let mut pc = 0;
    let mut relbase=0;
    let mut retval;

    loop {
        match code[pc] % 100 {
            1 => { // ADD
                let pidx = prepare_param_idx(3, pc, relbase, &code);
                code[pidx.2] = code[pidx.0] + code[pidx.1];
                pc += 4;
            },

            2 => { // MUL
                let pidx = prepare_param_idx(3, pc, relbase, &code);
                code[pidx.2] = code[pidx.0] * code[pidx.1];
                pc += 4;
            },

            3 => { // IN
                let pidx = prepare_param_idx(1, pc, relbase, &code);
                code[pidx.0] = input.recv().unwrap() as i64;

                pc += 2;
            },
            4 => { // OUT
                let pidx = prepare_param_idx(1, pc, relbase, &code);
                retval = code[pidx.0] as i32;
                output.send(retval);
                pc += 2;
            },
            5 => { // jump-if-true
                let pidx = prepare_param_idx(2, pc, relbase, &code);
                if code[pidx.0] != 0 { pc = code[pidx.1] as usize; } else { pc += 3; }
            },
            6 => { //jump-if-false
                let pidx = prepare_param_idx(2, pc, relbase, &code);
                if code[pidx.0] == 0 { pc = code[pidx.1] as usize; } else { pc += 3; }
            },
            7 => { // lt
                let pidx = prepare_param_idx(3, pc, relbase, &code);
                if code[pidx.0] < code[pidx.1] { code[pidx.2] = 1; } else { code[pidx.2] = 0; }
                pc += 4;
            },
            8 => { // eq
                let pidx = prepare_param_idx(3, pc, relbase, &code);
                if code[pidx.0] == code[pidx.1] { code[pidx.2] = 1; } else { code[pidx.2] = 0; }
                pc += 4;
            },
            9 => {
                let pidx = prepare_param_idx(1, pc, relbase, &code);
                relbase = (relbase as i64 + code[pidx.0]) as usize;
                pc += 2;
            },
            99 => break,
            _ => {
                println!("Invalid Opcode pc: {} - {}",  pc, code[pc] );
                break;
            }
        }
    }
}

fn prepare_param_idx(numparam: usize, pc: usize, relbase: usize, code: &Vec<i64> ) -> (usize, usize, usize)
{
    let mut pmask = (code[pc] / 100) as usize;
    let mut param = [0,0,0];
    for i in 0..numparam {
        param[i] = match pmask % 10 {
            1 => pc + i+1,
            2 => (code[pc + i+1] + relbase as i64) as usize,
            _ => code[pc + i+1] as usize,
        };
        pmask /= 10;
    }
    (param[0], param[1], param[2])
}