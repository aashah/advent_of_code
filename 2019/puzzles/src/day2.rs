use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day2.txt";
    let input = fs::read_to_string(filename)?;

    let mut program: Vec<u32> = input
        .split(',')
        .map(|entry| entry.trim())
        .map(|instruction| -> u32 { instruction.parse::<u32>().expect("invalid instruction") })
        .collect();

    // go into 1202 state
    program[1] = 12;
    program[2] = 2;

    run(&mut program);

    println!("Value at position 0: {}", program[0]);
    Ok(())
}

fn binary_op(pc: usize, program: &mut Vec<u32>, f: fn(u32, u32) -> u32) -> usize {
    let op1 = program[pc + 1] as usize;
    let op2 = program[pc + 2] as usize;
    let store_addr = program[pc + 3] as usize;
    program[store_addr] = f(program[op1], program[op2]);

    pc + 4
}

fn run(program: &mut Vec<u32>) {
    let mut pc = 0;

    loop {
        let instruction = program[pc];

        match instruction {
            1 => {
                pc = binary_op(pc, program, { |val1, val2| val1 + val2 });
            }
            2 => {
                pc = binary_op(pc, program, { |val1, val2| val1 * val2 });
            }
            99 => break,
            _ => panic!("bad instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        super::run(&mut program);
        assert_eq!(program, vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50));
    }
}
