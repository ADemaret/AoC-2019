use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

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
    let org_prog = get_input(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut prog = org_prog.clone();
            prog.insert(1, noun);
            prog.insert(2, verb);
            run_prog(&mut prog);

            if let Some(val) = prog.get(&0).copied() {
                if val == 19690720 {
                    return Some(noun * 100 + verb);
                }
            }
        }
    }
    None
}

fn run_prog(prog: &mut HashMap<usize, usize>) {
    let mut pos = 0;
    while pos < prog.len() {
        match prog.get(&pos).copied() {
            Some(1) => {
                let val1 = prog.get(&(pos + 1)).unwrap();
                let val2 = prog.get(&(pos + 2)).unwrap();
                let val3 = *prog.get(&(pos + 3)).unwrap();
                prog.insert(val3, prog.get(val1).unwrap() + prog.get(val2).unwrap());
                // println!(
                //     "at pos {pos}, set prog[{val3}] to {}",
                //     prog.get(&val3).unwrap()
                // );
            }
            Some(2) => {
                let val1 = prog.get(&(pos + 1)).unwrap();
                let val2 = prog.get(&(pos + 2)).unwrap();
                let val3 = *prog.get(&(pos + 3)).unwrap();
                prog.insert(val3, prog.get(val1).unwrap() * prog.get(val2).unwrap());
                // println!(
                //     "at pos {pos}, set prog[{val3}] to {}",
                //     prog.get(&val3).unwrap()
                // );
            }
            Some(99) => {
                // println!("at pos {pos}, the prog ends");
                break;
            }
            _ => {}
        }
        pos += 4;
    }
}

fn get_input(input: &str) -> HashMap<usize, usize> {
    input
        .split(",")
        .enumerate()
        .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
        .collect::<HashMap<usize, usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            Some(6979)
        );
    }
}
