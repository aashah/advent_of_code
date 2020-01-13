use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day2.txt";
    let input = fs::read_to_string(filename)?;

    let program: Vec<u32> = input
        .split(',')
        .map(|entry| entry.trim())
        .map(|instruction| -> u32 { instruction.parse::<u32>().expect("invalid instruction") })
        .collect();

    for i in 0..100 {
        for j in 0..100 {
            let mut new_program = program.clone();
            new_program[1] = i;
            new_program[2] = j;
            run(&mut new_program);

            if new_program[0] == 19690720 {
                println!("Found the answer at ({}, {})", i, j);
                println!("100 * noun + verb = {}", 100 * i + j);
                return Ok(());
            }
        }
    }

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
