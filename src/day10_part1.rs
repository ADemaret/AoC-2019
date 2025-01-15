use std::{collections::HashSet, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo1.txt");
    let input = include_str!("../assets/day10_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    let grid = Grid2D::new(input);
    let asters = grid.get_vec_of_char_positions('#');

    let mut max = 0;
    for l in 0..grid.max_l {
        for c in 0..grid.max_c {
            if asters.contains(&(l, c)) {
                let mut lines = HashSet::new();
                // print!("at {l},{c} :");
                for a in &asters {
                    if *a != (l, c) {
                        let angle = math::get_angle((l, c), *a);
                        lines.insert((angle * 100_000.0).round() as isize);
                    }
                }
                // println!(" => {}", lines.len());
                if lines.len() > max {
                    max = lines.len();
                }
            }
        }
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo1.txt")),
            Some(8)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo2.txt")),
            Some(33)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo3.txt")),
            Some(35)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo4.txt")),
            Some(41)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo5.txt")),
            Some(210)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input.txt")),
            Some(329)
        );
    }
}
