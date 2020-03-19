use std::{fs, thread};
use std::sync::mpsc::{channel, Sender, Receiver};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut vacubot: Vec<i64> = filecontents.trim().split(",").map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    vacubot.resize(vacubot.len() + 100_000, 0);

    let (_tobot, fromparent) = channel();
    let (toparent, frombot) = channel();
    let mut runner = vacubot.clone();
    thread::spawn(move || { run(fromparent, toparent, &mut runner); });

    let mut view= [[' ';55]; 50];

    let mut y = 0;
    let mut x = 0;

    let mut robot: Robot = Robot { x: 0, y:0, heading: Heading {dx: 0, dy:0 }};
    loop {
        match frombot.recv() {
            Ok(pix) => match (pix as u8) as char {
                '\n' => { y+=1; x=0; },
                '^' => { robot = Robot { x, y, heading: Heading {dy: -1, dx:0 }};
                         view[y][x] = (pix as u8) as char; x+= 1; },
                'v' => { robot = Robot { x, y, heading: Heading {dy: 1, dx:0 }};
                        view[y][x] = (pix as u8) as char; x+= 1; },
                '<' => { robot = Robot { x, y, heading: Heading {dy: 0, dx:-1 }};
                        view[y][x] = (pix as u8) as char; x+= 1; },
                '>' => { robot = Robot { x, y, heading: Heading {dy: 0, dx:1 }};
                        view[y][x] = (pix as u8) as char; x+= 1; },
                _  => { view[y][x] = (pix as u8) as char; x+=1; },
            }
            _ => break
        };
    }

    // Draw Maze
    for line in view.iter() {
        for chr in line.iter() {
            print!("{}", chr);
        }
        println!();
    }

    // Part 1
    let mut alignment_parameters = 0;
    for y in 1 .. view.len()-1 {
        for x in 1 .. view[y].len()-1 {
            if  view[y][x] != '.' && view[y-1][x] != '.' && view[y+1][x] != '.' && view[y][x+1] != '.' && view[y][x-1] != '.' {
                alignment_parameters += x*y;
            }
        }
    }

    println!("Alignment Params: {}", alignment_parameters);

    let mut path :Vec<(char,Robot)> = Vec::new();

    let mut heading = robot.heading;
    while heading != (Heading {dx:0, dy:0}) {
        heading = robot.get_direction(&view);

        if robot.heading.left() == heading { path.push(('L',robot));}
        else if robot.heading.right() == heading { path.push(('R',robot)); }
        robot.heading = heading; robot.go();
    }

    path.push(('E',robot));

    let mut segments:String = String::new();

    for i in 1..path.len() {
        let distance = (path[i-1].1.x as i32 - path[i].1.x as i32 + path[i-1].1.y as i32 - path[i].1.y as i32).abs();
        segments.push(path[i-1].0);
        segments.push(',');
        segments.push_str( &distance.to_string());
        segments.push(',');
    }
    segments.pop();
    println!("Path: {:?}", segments);

    /* Manually matched
        A: L,12,L,8,L,8
        B: L,12,R,4,L,12,R,6
        C: R,4,L,12,L,12,R,6

        A,B,A,C,B,A,C,A,C,B
    */
    let main ="A,B,A,C,B,A,C,A,C,B\nL,12,L,8,L,8\nL,12,R,4,L,12,R,6\nR,4,L,12,L,12,R,6\nn\n";

    let (tobot, fromparent) = channel();
    let (toparent, frombot) = channel();
    vacubot[0]=2;

    thread::spawn(move || { run(fromparent, toparent, &mut vacubot.clone()); });

    for c in main.chars() {
        tobot.send(c as i32);
    }

    loop {
        match frombot.recv() {
            Ok(pix) => {
                if pix > 256 {
                    println!("{}", pix);
                }
            }
            _ => break
        };
    }
}

fn within_bounds(x: i32, y: i32, view: &[[char;55]; 50] ) -> bool
{
    x >= 0 && x < view[0].len() as i32 && y >= 0 && y < view.len() as i32
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Heading {
    dx: i32,
    dy: i32
}
impl Heading {
    fn left(&self) -> Heading {
        match self {
            Heading {dx: 0, dy: 1} => Heading {dx:1, dy: 0}, Heading {dx:  0, dy: -1} => Heading {dx:-1, dy:0},
            Heading {dx: 1, dy: 0} => Heading {dx:0, dy:-1}, Heading {dx: -1, dy:  0} => Heading {dx: 0, dy:1},
            _ => Heading {dx:0 , dy:0 }
        }
    }

    fn right(&self) -> Heading {
        match self {
            Heading {dx: 0, dy: 1} => Heading {dx:-1, dy:0}, Heading {dx:  0, dy: -1} => Heading {dx:1, dy: 0},
            Heading {dx: 1, dy: 0} => Heading {dx: 0, dy:1}, Heading {dx: -1, dy:  0} => Heading {dx:0, dy:-1},
            _ => Heading {dx:0 , dy:0 }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    x: usize,
    y: usize,
    heading: Heading
}
impl Robot {
    fn get_direction(&self, view: &[[char;55]; 50]) -> Heading {

        let mut heading = Heading {dx:0, dy:0};
        if self.on_path(self.heading, view) { heading = self.heading; }
        else if self.on_path(self.heading.left(), view) { heading = self.heading.left(); }
        else if self.on_path(self.heading.right(), view) { heading = self.heading.right(); }

        heading
    }

    fn on_path(&self, heading: Heading, view: &[[char;55]; 50]) -> bool
    {
        within_bounds(self.x as i32 + heading.dx,self.y as i32 + heading.dy, view) &&
        view[(self.y as i32 + heading.dy) as usize][(self.x as i32 + heading.dx) as usize] == '#'
    }

    fn go(&mut self)
    {
        self.x = (self.x as i32 + self.heading.dx) as usize;
        self.y = (self.y as i32 + self.heading.dy) as usize;
    }
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