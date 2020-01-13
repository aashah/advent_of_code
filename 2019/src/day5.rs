use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day5.txt";
    let input = fs::read_to_string(filename)?;

    let mut program: Vec<i32> = input
        .split(',')
        .map(|entry| entry.trim())
        .map(|instruction| -> i32 { instruction.parse::<i32>().expect("invalid instruction") })
        .collect();

    run(&mut program, 1);

    Ok(())
}

fn binary_op(pc: usize, program: &mut Vec<i32>, modes: &Vec<Mode>, f: fn(i32, i32) -> i32) {
    let op1 = program[pc + 1];
    let op2 = program[pc + 2];
    let store_addr = program[pc + 3] as usize;

    let param1 = foo(program, op1, &modes[0]);
    let param2 = foo(program, op2, &modes[1]);
    program[store_addr] = f(param1, param2);
}

fn foo(program: &Vec<i32>, val: i32, mode: &Mode) -> i32 {
    return match mode {
        Mode::Position => program[val as usize],
        Mode::Immediate => val,
    };
}

fn parse_instruction(opcode: i32) -> Instruction {
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

fn run(program: &mut Vec<i32>, input: i32) {
    let mut pc = 0;

    loop {
        let instruction = parse_instruction(program[pc]);

        match instruction.op {
            1 => {
                binary_op(pc, program, &instruction.modes, {
                    |val1, val2| val1 + val2
                });
                pc += 4;
            }
            2 => {
                binary_op(pc, program, &instruction.modes, {
                    |val1, val2| val1 * val2
                });
                pc += 4;
            }
            3 => {
                let param = program[pc + 1];
                program[param as usize] = input;

                pc += 2;
            }
            4 => {
                let val = foo(program, program[pc + 1], &instruction.modes[0]);
                println!("{}", val);
                pc += 2;
            }
            99 => break,
            _ => panic!("bad instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let mut program = vec![1002, 4, 3, 4, 33];
        super::run(&mut program, 1);
        assert_eq!(program, vec!(1002, 4, 3, 4, 99));
    }

    #[test]
    fn simple_allows_negatives() {
        let mut program = vec![1101, 100, -1, 4, 0];
        super::run(&mut program, 1);
        assert_eq!(program, vec!(1101, 100, -1, 4, 99));
    }

    #[test]
    fn simple_input() {
        let mut program = vec![3, 1, 99];
        super::run(&mut program, 5);
        assert_eq!(program, vec!(3, 5, 99));
    }
}
