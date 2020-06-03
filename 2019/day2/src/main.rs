use std::fs;

fn main() {
    let program = fs::read_to_string("input.txt")
        .expect("Cannot open [input.txt]")
        .split(",")
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let verb:u32 = 12;
    let noun:u32 = 2;
    let result = intcode(verb,noun,&mut program.clone());
    println!("Part 1: {}", result);

    for verb in 0..99 {
        for noun in 0..99 {
            if intcode(verb, noun,&mut program.clone()) == 19690720 {
                println!("Part 2: Verb: {} Noun: {} - {}", verb, noun, 100*verb+noun);
                break;
            }
        }
    }
}

fn intcode(verb: u32, noun: u32, program: &mut Vec<u32>) -> u32 {
    let mut instruction_pointer = 0;

    program[1]=verb;
    program[2]=noun;
    loop {
        let opcode = program[instruction_pointer];
        match opcode {
            1 => { instruction_pointer += add(instruction_pointer, program); },
            2 => { instruction_pointer += mul(instruction_pointer, program); },
            99 => break,
            _ => { panic!("Invalid Opcode"); },
        }
    }
    program[0]
}

fn add(pc: usize, memory: &mut Vec<u32>) -> usize {
    let (p1,p2,p3) = fetch_parameters(pc, memory);
    memory[p3] = memory[p1] + memory[p2];
    4
}

fn mul(pc: usize, memory: &mut Vec<u32>) -> usize {
    let (p1,p2,p3) = fetch_parameters(pc, memory);
    memory[p3] = memory[p1] * memory[p2];
    4
}

fn fetch_parameters(pc:usize, memory: &Vec<u32>) -> (usize, usize, usize) {
    ( memory[pc + 1] as usize, memory[pc + 2] as usize, memory[pc + 3] as usize )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_parameters(){
        let memory:Vec<u32> = vec![1,2,3,4];
        assert_eq!(fetch_parameters(0, &memory), (2, 3, 4));
    }

    #[test]
    fn test_add(){
        let mut memory:Vec<u32> = vec![1,4,5,6,5,5,0];
        assert_eq!(add(0, &mut memory),4);
        assert_eq!(memory, vec![1,4,5,6,5,5,10])
    }

    #[test]
    fn test_mul(){
        let mut memory:Vec<u32> = vec![2,4,5,6,5,5,0];
        assert_eq!(mul(0, &mut memory),4);
        assert_eq!(memory, vec![2,4,5,6,5,5,25])
    }
    #[test]
    fn test_intcode_testprogram_1() {
        let mut testprogram: Vec<u32> = vec![1,0,0,0,99];
        let expected_result: Vec<u32> = vec![2,0,0,0,99];
        intcode(0,0,&mut testprogram);
        assert_eq!(testprogram, expected_result);
    }

    #[test]
    fn test_intcode_testprogram_2() {
        let mut testprogram: Vec<u32> = vec![2,3,0,3,99];
        let expected_result: Vec<u32> = vec![2,3,0,6,99];
        intcode(3,0,&mut testprogram);
        assert_eq!(testprogram, expected_result);
    }
    #[test]
    fn test_intcode_testprogram_3() {
        let mut testprogram: Vec<u32> = vec![2,4,4,5,99,0];
        let expected_result: Vec<u32> = vec![2,4,4,5,99,9801];
        intcode(4,4,&mut testprogram);
        assert_eq!(testprogram, expected_result);
    }
    #[test]
    fn test_intcode_testprogram_4() {
        let mut testprogram: Vec<u32> = vec![1,1,1,4,99,5,6,0,99];
        let expected_result: Vec<u32> = vec![30,1,1,4,2,5,6,0,99];
        intcode(1,1,&mut testprogram);
        assert_eq!(testprogram, expected_result);
    }
    #[test]
    fn test_intcode_testprogram_5() {
        let mut testprogram: Vec<u32> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected_result: Vec<u32> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        intcode(9,10,&mut testprogram);
        assert_eq!(testprogram, expected_result);
    }
}