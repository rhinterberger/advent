use std::{fs, thread};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::{HashMap};
use std::ops::Add;

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut code: Vec<i64> = filecontents.trim().split(",").map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    code.resize(code.len()+100_000, 0);

    let (tocode, fromparent) = channel();
    let (toparent, fromcode) = channel();

    let mut amplifier_code = code.clone();
    thread::spawn(move || {
        run(fromparent, toparent, &mut amplifier_code);
    });

    let mut direction = Point {x:0,y:-1}; // up
    let mut panels = HashMap::new();
    let mut position = Point {x:0,y:0};
    panels.entry(position).or_insert(1);
    loop {
        tocode.send(*panels.get(&position).unwrap());

        let color:i32 = match fromcode.recv() { Ok(col) => col, _ => break};
        let change_direction:i32 = match fromcode.recv() { Ok(dir) => dir, _ => break };

        match change_direction {
            0 => match direction { // Turn Left
                Point {x:0,y:1}  =>  direction = Point {x:-1, y:0},
                Point {x:1,y:0}  => direction = Point {x:0, y:1},
                Point {x:0,y:-1} => direction = Point {x:1, y:0},
                Point {x:-1,y:0} => direction = Point {x:0, y:-1},
                _ => {}
            }
            1 => match direction { // Turn Right
                Point { x: 0, y: 1 } => direction = Point { x: 1, y: 0 },
                Point { x: 1, y: 0 } => direction = Point { x: 0, y: -1 },
                Point { x: 0, y: -1 } => direction = Point { x: -1, y: 0 },
                Point { x: -1, y: 0 } => direction = Point { x: 0, y: 1 },
                _ => {}
            },
            _ => {}

        }

        panels.insert(position, color);
        position = position.add(direction);
        panels.entry(position).or_insert(0);
    }

    let mut image: Vec<u8> = Vec::new();
    image.resize(252,' ' as u8);

    for (panel, color) in panels {
        image[(panel.x.abs()+panel.y*42) as usize]= match color {1=> '*' as u8, _ => ' ' as u8};
    }

    let imagestr = String::from_utf8(image.clone()).unwrap();
    for image_line in (0..imagestr.len()).step_by(42) {
        println!("{:?}", &imagestr[image_line..image_line+42]);
    }


}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn run(input: Receiver<i32>, output: Sender<i32>, code: &mut Vec<i64>)
{
    let mut pc = 0;
    let mut relbase=0;
    let mut retval = 0;

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