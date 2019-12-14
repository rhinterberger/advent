use std::{fs, io};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut code  = filecontents.split(",").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    run(&mut code);
}


pub fn run(code: &mut Vec<i32>)
{
    let mut pc = 0;

    loop {
        let pmask = code[pc] / 100;
        // Panik at end of code if pc + x > code.len()
        let p1_index = if (pmask & 1  ) == 0 { code[pc+1] as usize } else { pc + 1 as usize };
        let p2_index = if (pmask & 10 ) == 0 { code[pc+2] as usize } else { pc + 2 as usize };
        let p3_index = if (pmask & 100) == 0 { code[pc+3] as usize } else { pc + 3 as usize };

        match code[pc] % 100 {
            1 => { // ADD
                code[p3_index] = code[p1_index] + code[p2_index];
                pc += 4;
            },

            2 => { // MUL
                code[p3_index] = code[p1_index] * code[p2_index];
                pc += 4;
            },

            3 => { // IN
                let mut input = String::new();
                io::stdin().read_line(&mut input);
                input.pop();   // Remove \n

                let p = code[pc+1] as usize;
                code[p as usize ]  =  input.parse::<i32>().unwrap();

                pc += 2;
            },
            4 => { // OUT
                println!("{}", code[code[pc+1 as usize] as usize]); pc += 2;
            },
            5 => { // jump-if-true
                if code[p1_index] != 0 { pc = code[p2_index] as usize; } else { pc += 3; }
            },
            6 => { //jump-if-false
                if code[p1_index] == 0 { pc = code[p2_index] as usize; } else { pc += 3; }
            },
            7 => { // lt
                if code[p1_index] < code[p2_index] { code[p3_index] = 1; } else { code[p3_index] = 0; }
                pc += 4;
            },
            8 => { // eq
                if code[p1_index] == code[p2_index] { code[p3_index] = 1; } else { code[p3_index] = 0; }
                pc += 4;
            },
            99 => break,
            _ => {
                println!("Invalid Opcode pc: {} - {}",  pc, code[pc] );
                break;
            },
        }
    }
}