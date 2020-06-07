pub struct Intcode {
    program: Vec<i32>,
    instruction_pointer: usize,
    input: i32
}
impl Intcode {
    pub fn new(program: Vec<i32>, input: i32) -> Intcode {
        Intcode { program, input, instruction_pointer: 0 }
    }

    pub fn run(&mut self) {
        loop {
            match self.decode_instruction() {
                1 => self.execute(Intcode::add, 3, 4),
                2 => self.execute(Intcode::multiply, 3, 4),
                3 => self.execute(Intcode::input, 1, 2),
                4 => self.execute(Intcode::output,1, 2),
                5 => self.execute(Intcode::jump_if_true, 2, 0),
                6 => self.execute(Intcode::jump_if_false,2, 0),
                7 => self.execute(Intcode::lower_than,3, 4),
                8 => self.execute(Intcode::equal, 3, 4),
                99 => break,
                _ => panic!("Invalid instruction")
            }
        }
    }

    fn decode_instruction(&self) -> i32 {
        self.program[self.instruction_pointer] % 100
    }

    fn execute<F>(&mut self, method: F, num_params: usize, increment: usize)
        where F : Fn(&mut Self, Vec<usize>)
    {
        let params = self.parameter_address(num_params);
        method(self, params);
        self.increment_instruction_pointer(increment);
    }

    fn parameter_address(&self, num_parameters: usize) -> Vec<usize> {
        let mut params: Vec<usize> = Vec::new();

        for parameter_number in 0..num_parameters {
            let mut parameter_address = self.instruction_pointer + 1 + parameter_number;
            if self.decode_parameter_mode(parameter_number) == 0 {
                parameter_address = self.program[parameter_address] as usize;
            }
            params.push(parameter_address);
        }
        params
    }

    fn decode_parameter_mode(&self, parameter: usize) -> i32 {
        (self.program[self.instruction_pointer] / 100) & 10_i32.pow(parameter as u32)
    }

    fn increment_instruction_pointer(&mut self, increment: usize) {
        self.instruction_pointer += increment;
    }

    fn add(&mut self, params: Vec<usize>) {
        self.program[params[2]] = self.program[params[0]] + self.program[params[1]];
    }

    fn multiply(&mut self, params: Vec<usize>) {
        self.program[params[2]] = self.program[params[0]] * self.program[params[1]];
    }

    fn input(&mut self, params: Vec<usize>) {
        self.program[params[0]] = self.input;
    }

    fn output(&mut self, params: Vec<usize>) {
        println!("{}", self.program[params[0]]);
    }

    fn jump_if_true(&mut self, params: Vec<usize>) {
        if self.program[params[0]] != 0 {
            self.jump(self.program[params[1]]);
        }
        else {
            self.increment_instruction_pointer(3);
        }
    }

    fn jump_if_false(&mut self, params: Vec<usize>) {
        if self.program[params[0]] == 0 {
           self.jump(self.program[params[1]]);
        } else {
            self.increment_instruction_pointer(3);
        }
    }

    fn jump(&mut self, destination: i32) {
        self.instruction_pointer = destination as usize;
    }

    fn lower_than(&mut self, params: Vec<usize>) {
        self.program[params[2]] = (self.program[params[0]] < self.program[params[1]]) as i32;
    }

    fn equal(&mut self, params: Vec<usize>) {
        self.program[params[2]] = (self.program[params[0]] == self.program[params[1]]) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_location() {
        let mut comp = Intcode::new([1,5,6,7,20,21,22,23].to_vec());
        // immediate Mode
        assert_eq!(comp.parameter_address(1, 1), [1]);
        assert_eq!(comp.parameter_address(2, 11), [1,2]);
        assert_eq!(comp.parameter_address(3, 111), [1,2,3]);
        // position Mode
        assert_eq!(comp.parameter_address(1, 0), [21]);
        assert_eq!(comp.parameter_address(2, 0), [21,22]);
        assert_eq!(comp.parameter_address(3, 0), [21,22,23]);
    }
}