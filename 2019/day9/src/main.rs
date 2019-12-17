use std::{fs};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut code: Vec<i64> = filecontents.trim().split(",").map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    code.resize(code.len() + 34463338, 0);
    run(1, &mut code.clone());
    run(2, &mut code.clone());
}

pub fn run(input: i64, code: &mut Vec<i64>)
{
    let mut pc = 0;
    let mut relbase=0;

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
                code[ pidx.0 ] = input;
                pc += 2;
            },
            4 => { // OUT
                let pidx = prepare_param_idx(1, pc, relbase, &code);
                println!("{}", code[pidx.0]); pc += 2;
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
            },
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