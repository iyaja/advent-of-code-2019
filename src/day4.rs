use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split("-")
        .map(|l| {
            l.parse::<usize>().unwrap()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn check_simple(input: &[u8]) -> usize {
    let mut valid = 0;
    
    for trial in 372037..905157 {
        let pass = trial.to_string();
        let mut doubleCount = 0;
        for i in 1..pass.len() {
            if pass.chars().nth(i) == pass.chars().nth(i - 1) {
                doubleCount += 1;
            }
        }

        if doubleCount == 0 {
            continue;
        }

        let mut ascending = true;
        for i in 1..pass.len() {
            if pass.chars().nth(i) < pass.chars().nth(i - 1) {
                ascending = false
            }
        }
        if !ascending {
            continue;
        }

        valid += 1;
    }
    valid
}

#[aoc(day4, part2)]
pub fn check(input: &[u8]) -> usize {
    let mut valid = 0;
    for trial in 372037..905157 {
        let pass = trial.to_string();
        if (validate(&pass)) {
            valid += 1;
        }
        
    }
    valid
}

pub fn validate(input: &str) -> bool {
    
    let pass = input.to_string();
    let mut counts = 1;
    let mut idx = 0;
    let mut got_double = false;
    for mut _i in 0..pass.len()-1 {
        for j in 1..pass.len() {
            if pass.chars().nth(idx) == pass.chars().nth(idx + j) {
                counts += 1;
            } else {
                break;
            }
        }
        if counts == 2 {
          got_double = true;
        }
        
        idx+=counts;
        counts = 1;
        if idx >= pass.len()-1 {
          break;
        }
    }

    let mut ascending = true;
    for i in 1..pass.len() {
        if pass.chars().nth(i) < pass.chars().nth(i - 1) {
            ascending = false
        }
    }
    ascending && got_double
}



