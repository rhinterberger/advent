use std::{fs, thread};
use itertools::{Itertools};
use std::sync::mpsc::{channel, Sender, Receiver};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let code = filecontents.split(",").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut max_result = 0;

    let perms = (0..5).permutations(5);
    for permutation in perms {
        let mut input0 = vec![0, permutation[0]];
        let mut input1 = vec![run(&mut input0, &mut code.clone()), permutation[1]];
        let mut input2 = vec![run(&mut input1, &mut code.clone()), permutation[2]];
        let mut input3 = vec![run(&mut input2, &mut code.clone()), permutation[3]];
        let mut input4 = vec![run(&mut input3, &mut code.clone()), permutation[4]];
        let result = run(&mut input4, &mut code.clone());

        if max_result <= result {
            max_result = result;
        }
    }
    println!("Part 1 : {:?}", max_result);

    max_result=0;
    let perms = (5..10).permutations(5);
    for permutation in perms {

        let (send_to_amp5, receive_in_amp5) = channel();
        let (send_to_amp4, receive_in_amp4) = channel();
        let (send_to_amp3, receive_in_amp3) = channel();
        let (send_to_amp2, receive_in_amp2) = channel();
        let (send_to_amp1, receive_in_amp1) = channel();
        let (send_to_parent, receive_in_parent) = channel();

        let parent_to_amp1 = send_to_amp1.clone();
        let parent_to_amp2 = send_to_amp2.clone();
        let parent_to_amp3 = send_to_amp3.clone();
        let parent_to_amp4 = send_to_amp4.clone();
        let parent_to_amp5 = send_to_amp5.clone();

        let amp1_to_parent = send_to_parent.clone();
        let amp2_to_parent = send_to_parent.clone();
        let amp3_to_parent = send_to_parent.clone();
        let amp4_to_parent = send_to_parent.clone();
        let amp5_to_parent = send_to_parent.clone();

        let mut amplifier_code = code.clone();
        thread::spawn(move || {
            run_thread(amp1_to_parent,send_to_amp2, receive_in_amp1, &mut amplifier_code);
        });

        let mut amplifier_code = code.clone();
        thread::spawn(move || {
            run_thread(amp2_to_parent, send_to_amp3, receive_in_amp2, &mut amplifier_code);
        });

        let mut amplifier_code = code.clone();
        thread::spawn(move || {
            run_thread(amp3_to_parent,send_to_amp4, receive_in_amp3, &mut amplifier_code);
        });

        let mut amplifier_code = code.clone();
        thread::spawn(move || {
            run_thread(amp4_to_parent,send_to_amp5, receive_in_amp4, &mut amplifier_code);
        });

        let mut amplifier_code = code.clone();
        thread::spawn(move || {
            run_thread(amp5_to_parent,send_to_amp1, receive_in_amp5, &mut amplifier_code);
        });

        parent_to_amp1.send(permutation[0]);
        parent_to_amp2.send(permutation[1]);
        parent_to_amp3.send(permutation[2]);
        parent_to_amp4.send(permutation[3]);
        parent_to_amp5.send(permutation[4]);

        parent_to_amp1.send(0);

        for _i in 0..5{
            let res=receive_in_parent.recv().unwrap();
            if max_result <= res {
                max_result = res;
            }
        }
    }
    println!("Part 2: {}", max_result);
}

pub fn run(input: &mut Vec<i32>, code: &mut Vec<i32>) -> i32
{
    let mut pc = 0;
    let mut retval =0;

    loop {
        let pmask = code[pc] / 100;

        let mut p1_index:usize = 0;
        let mut p2_index:usize = 0;
        let mut p3_index:usize = 0;

        if pc+1 < code.len() { p1_index = if (pmask & 1  ) == 0 { code[pc+1] as usize } else { pc + 1 as usize }; }
        if pc+2 < code.len() { p2_index = if (pmask & 10 ) == 0 { code[pc+2] as usize } else { pc + 2 as usize }; }
        if pc+3 < code.len() { p3_index = if (pmask & 100) == 0 { code[pc+3] as usize } else { pc + 3 as usize }; }

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
                let p = code[pc+1] as usize;
                code[p as usize ] = input.pop().unwrap();

                pc += 2;
            },
            4 => { // OUT
                retval = code[code[pc+1 as usize] as usize];
                pc += 2;
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
            99 => return retval,
            _ => {
                println!("Invalid Opcode pc: {} - {}",  pc, code[pc] );
                break;
            },
        }
    }
    retval
}

pub fn run_thread(parent: Sender<i32>, sender: Sender<i32>, receiver: Receiver<i32>, code: &mut Vec<i32>)
{
    let mut retval = 0;
    let mut pc = 0;
    loop {
        let pmask = code[pc] / 100;

        let mut p1_index: usize = 0;
        let mut p2_index: usize = 0;
        let mut p3_index: usize = 0;

        if pc + 1 < code.len() { p1_index = if (pmask & 1) == 0 { code[pc + 1] as usize } else { pc + 1 as usize }; }
        if pc + 2 < code.len() { p2_index = if (pmask & 10) == 0 { code[pc + 2] as usize } else { pc + 2 as usize }; }
        if pc + 3 < code.len() { p3_index = if (pmask & 100) == 0 { code[pc + 3] as usize } else { pc + 3 as usize }; }

        match code[pc] % 100 {
            1 => { // ADD
                code[p3_index] = code[p1_index] + code[p2_index];
                pc += 4;
            },

            2 => { // MUL
                code[p3_index] = code[p1_index] * code[p2_index];
                pc += 4;
            },

            3 => { // IN:
                let p = code[pc+1] as usize;
                code[p as usize] = receiver.recv().unwrap();

                pc += 2;
            },
            4 => { // OUT
                retval = code[code[pc + 1 as usize] as usize];
                sender.send(retval);
                pc += 2;
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
                println!("Invalid Opcode pc: {} - {}", pc, code[pc]);
                break;
            },
        }
    }
    parent.send(retval);
}
