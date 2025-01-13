use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day01_input_demo1.txt");
    let input = include_str!("../assets/day01_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|d| {
            let m = d.parse::<usize>().unwrap();
            (m / 3) - 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day01_input_demo1.txt")),
            34241
        );
        assert_eq!(
            get_answer(include_str!("../assets/day01_input.txt")),
            3246455
        );
    }
}
