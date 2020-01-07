#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        // .iter()
        .map(|c| {
            if c == '(' {
                1
            } else {
                -1
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(steps: &[i32]) -> i32 {
    steps.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(steps: &[i32]) -> usize {
    let mut floor: i32 = 0;
    for (idx, step) in steps.iter().enumerate() {
        floor += step;
        if floor == -1 {
            return idx + 1;
        }
    }
    0
}