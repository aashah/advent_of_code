use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day5.txt";
    let input = fs::read_to_string(filename)?;

    let memory: Vec<i32> = input
        .split(',')
        .map(|entry| entry.trim())
        .map(|instruction| -> i32 { instruction.parse::<i32>().expect("invalid instruction") })
        .collect();

    let mut program: Program = Program {
        program: memory,
        input: 1,
    };
    program.run();

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
}

impl Program {
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
                    println!("{}", val);
                    pc += 2;
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
        let mut program = super::Program {
            program: vec![1002, 4, 3, 4, 33],
            input: 1,
        };
        program.run();
        assert_eq!(program.program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn simple_allows_negatives() {
        let mut program = super::Program {
            program: vec![1101, 100, -1, 4, 0],
            input: 1,
        };
        program.run();
        assert_eq!(program.program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn simple_input() {
        let mut program = super::Program {
            program: vec![3, 1, 99],
            input: 1,
        };
        program.run();
        assert_eq!(program.program, vec!(3, 1, 99));
    }
}
