use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
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
    let mut prog = get_input(input);

    // println!("{}", prog_as_string(&prog));

    prog.insert(1, 12);
    prog.insert(2, 2);

    run_prog(&mut prog);
    // println!("{}", prog_as_string(&prog));

    prog.get(&0).copied()
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

    fn prog_as_string(prog: &HashMap<usize, usize>) -> String {
        let mut str = String::new();
        let max = prog.keys().max().unwrap();
        for index in 0..=*max {
            if let Some(val) = prog.get(&index) {
                str.push_str(format!("{val},").as_str());
            } else {
                str.push_str("-,");
            }
        }
        str
    }
 
    #[test]
    fn test_run() {
        let mut p = get_input("1,9,10,3,2,3,11,0,99,30,40,50");
        run_prog(&mut p);
        assert_eq!(prog_as_string(&p), "3500,9,10,70,2,3,11,0,99,30,40,50,");

        let mut p = get_input("1,0,0,0,99");
        run_prog(&mut p);
        assert_eq!(prog_as_string(&p), "2,0,0,0,99,");

        let mut p = get_input("2,3,0,3,99");
        run_prog(&mut p);
        assert_eq!(prog_as_string(&p), "2,3,0,6,99,");

        let mut p = get_input("2,4,4,5,99,0");
        run_prog(&mut p);
        assert_eq!(prog_as_string(&p), "2,4,4,5,99,9801,");

        let mut p = get_input("1,1,1,4,99,5,6,0,99");
        run_prog(&mut p);
        assert_eq!(prog_as_string(&p), "30,1,1,4,2,5,6,0,99,");
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            Some(3931283)
        );
    }
}
