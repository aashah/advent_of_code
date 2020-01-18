use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day5.txt";
    let input = fs::read_to_string(filename)?;

    let memory: Vec<i32> = input
        .split(',')
        .map(|entry| entry.trim())
        .map(|instruction| -> i32 { instruction.parse::<i32>().expect("invalid instruction") })
        .collect();

    let mut program = Program::new(memory, 5);
    program.run();
    println!("{:?}", program.output);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Instruction {
    op: i32,
    modes: Vec<Mode>,
}

struct Program {
    program: Vec<i32>,
    input: i32,
    output: Vec<i32>,
}

impl Program {
    fn new(program: Vec<i32>, input: i32) -> Program {
        Program {
            program,
            input,
            output: vec![],
        }
    }

    fn run(&mut self) {
        let mut pc = 0;

        loop {
            let instruction = self.parse_instruction(self.program[pc]);

            match instruction.op {
                1 => {
                    self.binary_op(pc, &instruction.modes, { |val1, val2| val1 + val2 });
                    pc += 4;
                }
                2 => {
                    self.binary_op(pc, &instruction.modes, { |val1, val2| val1 * val2 });
                    pc += 4;
                }
                3 => {
                    let param = self.program[pc + 1];
                    self.program[param as usize] = self.input;

                    pc += 2;
                }
                4 => {
                    let val = self.load(self.program[pc + 1], &instruction.modes[0]);
                    self.output.push(val);
                    pc += 2;
                }
                5 => {
                    // Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the
                    // instruction pointer to the value from the second parameter. Otherwise, it
                    // does nothing.
                    let val = self.load(self.program[pc + 1], &instruction.modes[0]);
                    if val != 0 {
                        pc = self.load(self.program[pc + 2], &instruction.modes[1]) as usize;
                        continue;
                    }

                    pc += 3;
                }
                6 => {
                    // Opcode 6 is jump-if-false: if the first parameter is zero, it sets the
                    // instruction pointer to the value from the second parameter. Otherwise, it
                    // does nothing.
                    let val = self.load(self.program[pc + 1], &instruction.modes[0]);
                    if val == 0 {
                        pc = self.load(self.program[pc + 2], &instruction.modes[1]) as usize;
                        continue;
                    }

                    pc += 3;
                }
                7 => {
                    // Opcode 7 is less than: if the first parameter is less than the second parameter,
                    // it stores 1 in the position given by the third parameter. Otherwise, it stores
                    // 0.
                    let op1 = self.load(self.program[pc + 1], &instruction.modes[0]);
                    let op2 = self.load(self.program[pc + 2], &instruction.modes[1]);
                    let store_addr = self.program[pc + 3] as usize;

                    if op1 < op2 {
                        self.program[store_addr] = 1;
                    } else {
                        self.program[store_addr] = 0;
                    }

                    pc += 4;
                }
                8 => {
                    // Opcode 8 is equals: if the first parameter is equal to the second parameter,
                    // it stores 1 in the position given by the third parameter. Otherwise, it stores
                    // 0.
                    let op1 = self.load(self.program[pc + 1], &instruction.modes[0]);
                    let op2 = self.load(self.program[pc + 2], &instruction.modes[1]);
                    let store_addr = self.program[pc + 3] as usize;

                    if op1 == op2 {
                        self.program[store_addr] = 1;
                    } else {
                        self.program[store_addr] = 0;
                    }

                    pc += 4;
                }
                99 => break,
                _ => panic!("bad instruction"),
            }
        }
    }

    fn binary_op(&mut self, pc: usize, modes: &Vec<Mode>, f: fn(i32, i32) -> i32) {
        let op1 = self.program[pc + 1];
        let op2 = self.program[pc + 2];
        let store_addr = self.program[pc + 3] as usize;

        let param1 = self.load(op1, &modes[0]);
        let param2 = self.load(op2, &modes[1]);
        self.program[store_addr] = f(param1, param2);
    }

    fn load(&self, val: i32, mode: &Mode) -> i32 {
        return match mode {
            Mode::Position => self.program[val as usize],
            Mode::Immediate => val,
        };
    }

    fn parse_instruction(&self, opcode: i32) -> Instruction {
        let op = opcode % 100;
        let remainder = (opcode - op) / 100;

        let mut modes = vec![];
        for c in remainder.to_string().chars().rev() {
            match c {
                '0' => modes.push(Mode::Position),
                '1' => modes.push(Mode::Immediate),
                _ => panic!("bad mode"),
            }
        }

        modes.resize(3, Mode::Position);

        return Instruction {
            op: op,
            modes: modes,
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let mut program = super::Program::new(vec![1002, 4, 3, 4, 33], 1);
        program.run();
        assert_eq!(program.program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn simple_allows_negatives() {
        let mut program = super::Program::new(vec![1101, 100, -1, 4, 0], 1);
        program.run();
        assert_eq!(program.program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn simple_input() {
        let mut program = super::Program::new(vec![3, 1, 99], 1);
        program.run();
        assert_eq!(program.program, vec!(3, 1, 99));
    }

    #[test]
    fn imode_simple_jump_if_true() {
        let mut program = super::Program::new(vec![1105, 1, 4, 99, 104, 42, 99], 1);
        program.run();
        assert_eq!(program.output, vec!(42));
    }

    #[test]
    fn imode_simple_jump_if_false() {
        let mut program = super::Program::new(vec![1106, 0, 4, 99, 104, 42, 99], 1);
        program.run();
        assert_eq!(program.output, vec!(42));
    }

    #[test]
    fn imode_less_than() {
        let mut program = super::Program::new(vec![1107, 40, 41, 5, 104, -1, 99], 1);
        program.run();
        assert_eq!(program.output, vec!(1));
    }

    #[test]
    fn imode_equals() {
        let mut program = super::Program::new(vec![1108, 42, 42, 5, 104, -1, 99], 1);
        program.run();
        assert_eq!(program.output, vec!(1));
    }

    #[test]
    fn imode_equals_ex1() {
        let mut program = super::Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 9);
        program.run();
        assert_eq!(program.output, vec!(0));
    }
}
