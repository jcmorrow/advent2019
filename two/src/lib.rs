extern crate four;
use four::int_parts;

use std::io;

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

struct ParameterModes {
    modes: Vec<ParameterMode>,
}

impl ParameterModes {
    fn get(&self, i: i32) -> ParameterMode {
        match self.modes.get(i as usize) {
            Some(x) => *x,
            None => ParameterMode::Position,
        }
    }
}

struct Instructions {
    head: usize,
    instructions: Vec<i32>,
}

fn parameter_mode_from_int(i: i32) -> ParameterMode {
    match i {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        _ => panic!("Invalid parameter mode: {}", i),
    }
}

fn parse_op_code(whole_opcode: i32) -> (i32, ParameterModes) {
    let opcode = whole_opcode % 100;
    let whole_opcode = whole_opcode / 100;
    let mut parameter_modes = int_parts(whole_opcode);
    parameter_modes.reverse();
    (
        opcode,
        ParameterModes {
            modes: parameter_modes
                .iter()
                .map(|x| parameter_mode_from_int(*x))
                .collect(),
        },
    )
}

impl Instructions {
    fn new(instructions: Vec<i32>) -> Instructions {
        Instructions {
            head: 0,
            instructions,
        }
    }

    fn read(&self) -> i32 {
        self.instructions[self.head]
    }

    fn peek(&self, i: i32) -> i32 {
        self.instructions[self.head + i as usize]
    }

    fn peek_at(&self, i: i32) -> i32 {
        self.instructions[i as usize]
    }

    fn write(&mut self, destination: i32, value: i32) {
        self.instructions[destination as usize] = value;
    }

    fn move_head(&mut self, i: usize) {
        self.head += i;
    }

    fn parameter_from(&self, i: i32, p: ParameterMode) -> i32 {
        match p {
            ParameterMode::Immediate => i,
            ParameterMode::Position => self.peek_at(i),
        }
    }

    fn evaluate(&mut self) {
        loop {
            match parse_op_code(self.read()) {
                (1, parameter_modes) => {
                    let destination = self.peek(3);
                    let x = self.parameter_from(self.peek(1), parameter_modes.get(0));
                    let y = self.parameter_from(self.peek(2), parameter_modes.get(1));
                    self.write(destination, x + y);
                    self.move_head(4);
                }
                (2, parameter_modes) => {
                    let destination = self.peek(3);
                    let x = self.parameter_from(self.peek(1), parameter_modes.get(0));
                    let y = self.parameter_from(self.peek(2), parameter_modes.get(1));
                    self.write(destination, x * y);
                    self.move_head(4);
                }
                (3, _parameter_modes) => {
                    let mut input = String::new();
                    let destination = self.peek(1);
                    println!("Please input something");
                    io::stdin().read_line(&mut input).unwrap();
                    println!("input: {}", input);
                    match input.trim().parse::<i32>() {
                        Ok(x) => self.write(destination, x),
                        Err(err) => {
                            panic!("Err: {}\nCould not parse as integer: {}", err, input.trim())
                        }
                    }
                    self.move_head(2);
                }
                (4, _parameter_modes) => {
                    println!("{}", self.peek_at(self.peek(1)));
                    self.move_head(2);
                }
                (99, _) => break,
                (opcode, _) => panic!("Not a valid OpCode: {}", opcode),
            }
        }
    }
}

pub fn compute(instructions: Vec<i32>) -> Vec<i32> {
    let mut instructions = Instructions::new(instructions);
    instructions.evaluate();
    instructions.instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(
            compute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(compute(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(compute(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(compute(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            compute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
