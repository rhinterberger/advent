use std::fs;

fn main() {
    let input = read_input("input.txt");

    // Part One
    let mut console = Console::new();
    console.parse_source(input.as_str());
    console.run();
    println!("{}",console.accumulator);

    // Part Two
    let mut console = Console::new();
    console.parse_source(input.as_str());
    let accumulator = part_two(&mut console);
    println!("{}",accumulator.unwrap());
}

fn part_two(console: &mut Console) -> Option<i32> {
    for i in 0..console.program.len() {
        let mut test_program = console.clone();
        swap_instructions(i, &mut test_program);
        test_program.run();
        if test_program.debug_state_stopped == false {
            return Some(test_program.accumulator);
        }
    }
    None
}

fn swap_instructions(i: usize, test_program: &mut Console) {
    let current_test_instruction = &test_program.program[i];
    match current_test_instruction.0.as_str() {
        "jmp" => { test_program.program[i] = ("nop".to_string(), current_test_instruction.1); },
        "nop" => { test_program.program[i] = ("jmp".to_string(), current_test_instruction.1); }
        _ => {}
    };
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
}

#[derive(Clone)]
struct Console {
    program: Vec<(String,i32)>,
    program_counter: i32,
    accumulator: i32,
    debug_list: Vec<i32>,
    debug_state_stopped: bool
}
impl Console {
    fn new() -> Console {
        Console {
            program: Vec::new(),
            program_counter: 0,
            accumulator: 0,
            debug_list: Vec::new(),
            debug_state_stopped: false
        }
    }

    fn run(&mut self) {
        while self.program_counter < self.program.len() as i32 {
            self.debug();
            if self.debug_state_stopped == true {
                return;
            }
            let instruction = self.program[self.program_counter as usize].clone();
            match instruction.0.as_str() {
                "acc" => self.acc(instruction.1),
                "jmp" => self.jmp(instruction.1),
                "nop" => self.nop(),
                _ => panic!("Invalid Instruction")
            }
        }
    }

    fn debug(&mut self) {
        if self.program_counter < 0 {
            println!("Program Counter underflow");
            self.debug_state_stopped = true;
        }

        if self.debug_list.contains(&self.program_counter) {
            self.debug_state_stopped = true;
        }
        self.debug_list.push(self.program_counter);
    }

    fn acc(&mut self, argument: i32) {
        self.accumulator = self.accumulator + argument;
        self.program_counter = self.program_counter + 1;
    }

    fn jmp(&mut self, argument: i32) {
        self.program_counter = self.program_counter  + argument;
    }

    fn nop(&mut self) {
        self.program_counter = self.program_counter  + 1;
    }

    fn parse_source(&mut self, source: &str) {
        source.lines().for_each(|line| {
            self.program.push(Console::parse_line(line))});

    }

    fn parse_line(line: &str) -> (String, i32) {
        let mut instruction = line.split_whitespace();

        (instruction.next().unwrap().to_string(), instruction.next().unwrap().parse::<i32>().unwrap())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulator() {
        let mut console = Console::new();
        console.acc(1);
        assert_eq!(console.accumulator, 1);
        console.acc(-1);
        assert_eq!(console.accumulator, 0);
    }

    #[test]
    fn jump() {
        let mut console = Console::new();
        console.jmp(10);
        assert_eq!(console.program_counter, 10);
    }

    #[test]
    fn nop() {
        let mut console = Console::new();
        console.nop();
        assert_eq!(console.program_counter, 1);
    }

    #[test]
    fn parse_program_line() {
        let line = "nop +0";
        assert_eq!(Console::parse_line(line), ("nop".to_string(),0));
    }

    #[test]
    fn parse_program() {
        let source = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let mut console = Console::new();
        console.parse_source(source);
        assert_eq!(console.program, vec![
            ("nop".to_string(),0),
            ("acc".to_string(),1),
            ("jmp".to_string(),4),
            ("acc".to_string(),3),
            ("jmp".to_string(),-3),
            ("acc".to_string(),-99),
            ("acc".to_string(),1),
            ("jmp".to_string(),-4),
            ("acc".to_string(),6)
        ]);
    }

    #[test]
    fn debug_flow() {
        let source = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let mut console = Console::new();
        console.parse_source(source);
        console.run();
        assert_eq!(console.accumulator, 5);
    }

}