use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

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
    let mut line1: HashMap<(isize, isize), usize> = HashMap::new();
    let mut cross: Vec<usize> = Vec::new();
    input.lines().enumerate().for_each(|(nr, line)| match nr {
        0 => {
            let mut current = (0, 0);
            let mut step1 = 1;
            line.split(",").for_each(|ch| {
                let dir = get_dir(&ch[0..1]);
                let len = ch[1..].parse::<usize>().unwrap();
                for _ in 0..len {
                    current.0 += dir.0;
                    current.1 += dir.1;
                    line1.entry(current).or_insert(step1);
                    step1 += 1;
                }
            })
        }
        1 => {
            let mut current = (0, 0);
            let mut step2 = 1;
            line.split(",").for_each(|ch| {
                let dir = get_dir(&ch[0..1]);
                let len = ch[1..].parse::<usize>().unwrap();
                for _ in 0..len {
                    current.0 += dir.0;
                    current.1 += dir.1;
                    if let Some(step1) = line1.get(&current) {
                        cross.push(step2 + step1);
                    }
                    step2 += 1;
                }
            })
        }
        _ => panic!("Too much lines"),
    });

    Some(*cross.iter().min().unwrap())
}

fn get_dir(code: &str) -> (isize, isize) {
    match code {
        "U" => (-1, 0),
        "D" => (1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => panic!("Unknon code"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input_demo1.txt")),
            Some(30)
        );
        assert_eq!(
            get_answer(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Some(610)
        );
        assert_eq!(
            get_answer(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Some(410)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day03_input.txt")),
            Some(107754)
        );
    }
}
