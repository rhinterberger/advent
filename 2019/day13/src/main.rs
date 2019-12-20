use std::{fs, thread};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::{ HashSet};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut arcade: Vec<i64> = filecontents.trim().split(",").map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    arcade.resize(arcade.len() + 10000, 0);

    let (toarcade, fromparent) = channel();
    let (toparent, fromarcade) = channel();

    let mut arcade_part1 = arcade.clone();
    thread::spawn(move || { run(fromparent, toparent, &mut arcade_part1); });

    let mut tiles = HashSet::new();

    loop {
        let mut tile = Tile {x:0,y:0, id:0};
        tile.x  = match fromarcade.recv() { Ok(col) => col, _ => break };
        tile.y  = match fromarcade.recv() { Ok(col) => col, _ => break };
        tile.id = match fromarcade.recv() { Ok(col) => col, _ => break };

        tiles.insert(tile);
    }

    let count = tiles.iter().fold(0, |acc, tile| match tile.id { 2 => acc + 1, _ => acc });
    println!("Part 1 Count: {}", count);

    let mut arcade_part2 = arcade.clone();
    arcade_part2[0] = 2;

    let (toarcade, fromparent) = channel();
    let (toparent, fromarcade) = channel();

    thread::spawn(move || { run(fromparent, toparent, &mut arcade_part2) });

    let mut paddle = Tile {x: 0, y: 0, id: 0};
    let mut score = 0;

    loop {
        let mut tile = Tile {x:0,y:0, id:0};
        tile.x  = match fromarcade.recv() { Ok(col) => col, _ => break };
        tile.y  = match fromarcade.recv() { Ok(col) => col, _ => break };
        tile.id = match fromarcade.recv() { Ok(col) => col, _ => break };

        match tile.id {
            3 => {paddle = tile; },
            4 => { toarcade.send(   (tile.x-paddle.x).signum()); },
            _ => {}
        }

        if tile.x == -1 && tile.y == 0 {
            score = tile.id;
        }
    }

    println!("Part 2 Score: {}", score)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Tile {
    x: i32, y:i32, id: i32
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