use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const VERBOSE: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
fn get_answer(input: &str) -> Option<isize> {
    let mut prog = get_input(input);
    let mut input = 1;

    run_prog(&mut prog, &mut input);

    Some(input)
}

fn run_prog(prog: &mut HashMap<usize, isize>, input: &mut isize) {
    let mut pos: usize = 0;
    while pos < prog.len() {
        let (modes, instruction) = parse_instruction(prog.get(&pos).unwrap());
        let mut len = 4;
        match instruction {
            Instructions::Add => {
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                prog.insert(out, v_in[0] + v_in[1]);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to {}", out, prog.get(&out).unwrap());
                }
            }
            Instructions::Mul => {
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                prog.insert(out, v_in[0] * v_in[1]);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to {}", out, prog.get(&out).unwrap());
                }
            }
            Instructions::In => {
                len = 2;
                let out = *prog.get(&(pos + 1)).unwrap() as usize;
                prog.insert(out, *input);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to input ({})", out, prog.get(&out).unwrap());
                }
            }
            Instructions::Out => {
                len = 2;
                let (v_in, _) = get_values(prog, pos, &modes, 0, 0, 0);
                *input = v_in[0];
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set output to {}", input);
                }
            }
            Instructions::End => {
                len = 1;
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> the prog ends");
                }
                break;
            }
        }
        pos += len;
    }
}

fn print_instruction(
    prog: &mut HashMap<usize, isize>,
    pos: usize,
    instruction: Instructions,
    len: usize,
) {
    print!("at pos {pos} : ");
    print!("{}({:?}),", prog.get(&pos).unwrap(), instruction);
    for i in pos + 1..pos + len {
        print!("{},", prog.get(&i).unwrap());
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

fn get_values(
    prog: &HashMap<usize, isize>,
    pos: usize,
    modes: &HashMap<usize, Mode>,
    in_min: usize,
    in_max: usize,
    out: usize,
) -> (Vec<isize>, usize) {
    let mut in_values = Vec::new();
    for index in in_min..=in_max {
        match modes.get(&index) {
            Some(Mode::Position) | None => {
                let val = *(prog.get(&(pos + index + 1)).unwrap()) as usize;
                in_values.push(*prog.get(&val).unwrap());
            }
            Some(Mode::Immediate) => {
                let val = *(prog.get(&(pos + index + 1)).unwrap());
                in_values.push(val);
            }
        }
    }
    let out_value = *(prog.get(&(pos + out + 1)).unwrap()) as usize;
    // println!("values in : {:?}", in_values);
    // println!("index out : {:?}", out_value);
    (in_values, out_value)
}

#[derive(Debug)]
enum Instructions {
    Add,
    Mul,
    In,
    Out,
    End,
}

fn parse_instruction(raw: &isize) -> (HashMap<usize, Mode>, Instructions) {
    let instruction = match raw % 100 {
        1 => Instructions::Add,
        2 => Instructions::Mul,
        3 => Instructions::In,
        4 => Instructions::Out,
        99 => Instructions::End,
        _ => panic!("Unknown instruction"),
    };
    let mut modes = HashMap::new();
    let str = raw.to_string();
    if *raw > 100 {
        let mut m = str.chars().take(str.len() - 2).collect::<Vec<char>>();
        m.reverse();
        for (i, mo) in m.iter().enumerate() {
            match mo {
                '0' => modes.insert(i, Mode::Position),
                '1' => modes.insert(i, Mode::Immediate),
                _ => panic!("Unknown mode"),
            };
        }
    }
    // println!("{str} => instruction {:?} and {:?}", instruction, modes);
    (modes, instruction)
}

fn get_input(input: &str) -> HashMap<usize, isize> {
    input
        .split(",")
        .enumerate()
        .map(|(i, x)| (i, x.parse::<isize>().unwrap()))
        .collect::<HashMap<usize, isize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day05_input.txt")),
            Some(7157989)
        );
    }
}
