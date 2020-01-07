use std::convert::TryInto;


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Segment, Segment) {
    input
        .lines()
        .map(|l| {
            l.split(",")
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn manhatten_distance_to_closest_intersection(input: &[usize]) -> usize {
    0
}

// #[aoc(day3, part2)]
// pub fn solve_part_2(input: &[usize]) -> usize {
//     return 0;
// }

pub fn manhatten_distance(p: (isize, isize), q: (isize, isize)) -> usize {
    ((p.0 - q.0) + (p.1 - q.1)).abs().try_into().unwrap()
}
