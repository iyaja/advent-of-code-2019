#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|l| {
            l.parse::<usize>().unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn compute(input: &[usize]) -> usize {
    let mut idx = 0;
    let mut seq = input.to_vec();
    // seq[1] = 12;
    // seq[2] = 2;
    loop {
        //println!("{}", idx);
        let opcode = seq[idx];
        let operand1 = seq[idx + 1];
        let operand2 = seq[idx + 2];
        let val = seq[idx + 3];
        if opcode == 1 {
            seq[val] = seq[operand1] + seq[operand2];
        } else if opcode == 2 {
            seq[val] = seq[operand1] * seq[operand2];
        } else if opcode == 99 {
            //println!("Halting at index {}", idx);
            break;
        } else {
            println!("Error! Opcode must be either 1 or 2! Got {} at index {}", opcode, idx);
        }

        idx += 4;
        if idx >= seq.len() {
            break;
        }
    }
    seq[0]
}

#[aoc(day2, part2)]
pub fn inverse(input: &[usize]) -> usize {
    let mut memory = input.to_vec();
    let mut code: String = "".to_owned();
    for noun in 0..99 {
        for verb in 0..99 {
            memory[1] = noun;
            memory[2] = verb;
            if compute(&memory) == 19690720 {
                println!("{}{}", noun, verb);
                code.push_str(&noun.to_string());
                code.push_str(&verb.to_string());
                return code.parse::<usize>().unwrap();
            }
        }
    }
    println!("No code found :(", );
    0
}