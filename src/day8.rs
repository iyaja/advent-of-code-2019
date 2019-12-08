extern crate ndarray;
use ndarray::prelude::*;

extern crate ndarray_stats;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .chars()
        
        .map(|c| {
            c.to_string().parse::<usize>().unwrap()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[usize]) -> usize {

    let arr = input.clone().to_vec();
    let num_images = arr.capacity() / (25 * 6);
    let images = Array::from_shape_vec((num_images, 6, 25), arr).unwrap();
    let mut min_zeros_idx = 0;
    let mut min_zeros = 100;
    for (n, image) in images.outer_iter().enumerate() {
        let zeros = ndarray_stats::DeviationExt::count_eq(&image, &Array::zeros((6, 25))).unwrap();
        if zeros < min_zeros {
            min_zeros = zeros;
            min_zeros_idx = n;
        }
    }

    let target_image = images.slice(s![min_zeros_idx, .., ..]);
    let ones = ndarray_stats::DeviationExt::count_eq(&target_image, &Array::ones((6, 25))).unwrap();
    let twos = ndarray_stats::DeviationExt::count_eq(&target_image, &Array::from_elem((6, 25), 2)).unwrap();
    ones * twos
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[usize]) -> Array<usize, Ix2> {
    let arr = input.clone().to_vec();
    let num_layers = arr.capacity() / (25 * 6);
    let layers = Array::from_shape_vec((num_layers, 6, 25), arr).unwrap();
    
    let mut decoded = Array::from_elem((6, 25), 2);

    for i in 0..6 {
        for j in 0..25  {
            for n in 0..100 {
                if decoded[[i, j]] == 2 {
                    decoded[[i, j]] = layers[[n, i, j]];
                } else {
                    break;
                }
            }
        }
    }

    decoded
}