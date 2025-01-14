use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

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
    let planets = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(")").unwrap();
            (b, a)
        })
        .collect::<HashMap<&str, &str>>();

    // for p in &planets {
    //     println!("{} {:?}", p.0, p.1);
    // }
    // println!("--");

    let mut result = 0;
    for p in &planets {
        let mut nbr = 1;
        // println!("{} {:?}", p.0, p.1);
        let mut next = p.1;
        loop {
            if *next != "COM" {
                nbr += 1;
            } else {
                // println!("{}", nbr);
                break;
            }
            next = planets.get(next).unwrap();
        }
        result += nbr;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo1.txt")),
            Some(42)
        );
        assert_eq!(get_answer(include_str!("../assets/day06_input.txt")), Some(200001));
    }
}
