// #[aoc_generator(day1)]
// pub fn input_generator(input: &str) -> Vec<usize> {
//     input
//         .lines()
//         .map(|l| {
//             l.parse::<usize>().unwrap()
//         })
//         .collect()
// }

// #[aoc(day1, part1)]
// pub fn solve_part1(input: &[usize]) -> f32 {
//     input
//         .iter()
//         .map(|&mass| {
//             (((mass as f32) / 3.0).floor() - 2.0)
//         })
//         .sum()
// }

// #[aoc(day1, part2)]
// pub fn solve_part2(input: &[usize]) -> f32 {
//     input
//         .iter()
//         .map(|&mass| {
//             calculate_fuel(mass as f32)
//         })
//         .sum()
// }