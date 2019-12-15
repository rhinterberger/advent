use std::fs;
use std::convert::TryInto;

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut code  = filecontents.split(",").map(|value| value.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    run(&mut code);
    println!("{}", code[0]);

    // Part 2
    let init_state = filecontents.split(",").map(|value| value.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    for verb in 0..99 {
        for noun in 0..99 {
            code = init_state.to_vec();
            code[1] = verb;
            code[2] = noun;
            run(&mut code);
            if code[0] == 19690720 {
                println!("{} : Verb: {} Noun: {} - {}", code[0], code[1], code[2], 100*code[1]+code[2]);
                break;
            }
        }
    }
}

fn run(code : &mut Vec<u32>)
{
    for pc in (0..code.len()).step_by(4)
    {
        let opcode = code[pc];
        let p1 :usize = code[pc+1].try_into().unwrap();
        let p2 :usize = code[pc+2].try_into().unwrap();
        let p3 :usize = code[pc+3].try_into().unwrap();

        if p1 >= code.len() || p2 >= code.len() || p3 >=code.len() {
            println!("Params out of Bounds {}", pc);
            break;
        }

        match opcode {
            1 => code[p3] = code[p1] + code[p2],
            2 => code[p3] = code[p1] * code[p2],
            99 => break,
            _ => { println!("Invalid Opcode"); break;},
        }
    }
}