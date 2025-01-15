use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const VERBOSE: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let input = include_str!("../assets/day07_input.txt");

    if let Some(answer) = get_answer(input, 0) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input_file: &str, input_value: isize) -> Option<isize> {
    let prog = get_input(input_file);
    let input = input_value;
    let mut max_thruster = 0;
    let mut max_comb = Vec::new();
    let comb = (0..5).permutations(5);
    'phase: for phase_comb in comb {
        if VERBOSE {
            println!("Phase = {:?}", phase_comb);
        }
        let mut this_input = input;
        for phase in &phase_comb {
            let mut this_prog = prog.clone();
            if run_prog(&mut this_prog, &mut this_input, *phase).is_err() {
                continue 'phase;
            }
        }
        if VERBOSE {
            println!("ouput is {}", this_input);
        }
        if this_input > max_thruster {
            max_thruster = this_input;
            max_comb = phase_comb.clone();
        }
    }
    if VERBOSE {
        println!("winning comb is {:?}", max_comb);
    }
    Some(max_thruster)
}

fn run_prog(
    prog: &mut HashMap<usize, isize>,
    input: &mut isize,
    phase: isize,
) -> Result<usize, usize> {
    let mut phase_used = false;
    let mut pos: usize = 0;
    while pos < prog.len() {
        let (modes, instruction) = parse_instruction(prog.get(&pos).unwrap());
        let mut len = 4;
        match instruction {
            Instructions::Add => {
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                let sum = v_in[0].saturating_add(v_in[1]);
                if sum == isize::MAX {
                    println!("overflow error");
                    return Err(1);
                }
                prog.insert(out, sum);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to {}", out, prog.get(&out).unwrap());
                }
                pos += len;
            }
            Instructions::Mul => {
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                let prod = v_in[0].saturating_mul(v_in[1]);
                if prod == isize::MAX {
                    println!("overflow error");
                    return Err(2);
                }
                prog.insert(out, prod);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to {}", out, prog.get(&out).unwrap());
                }
                pos += len;
            }
            Instructions::In => {
                len = 2;
                let out = *prog.get(&(pos + 1)).unwrap() as usize;
                if !phase_used {
                    prog.insert(out, phase);
                    phase_used = true;
                } else {
                    prog.insert(out, *input);
                }
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!(
                        "=> set prog[{}] to input ({})",
                        out,
                        prog.get(&out).unwrap()
                    );
                }
                pos += len;
            }
            Instructions::Out => {
                len = 2;
                let (v_in, _) = get_values(prog, pos, &modes, 0, 0, 0);
                *input = v_in[0];
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set output to {}", input);
                }
                pos += len;
            }
            Instructions::Jit => {
                len = 3;
                let (v_in, _) = get_values(prog, pos, &modes, 0, 1, 0);
                if v_in[0] != 0 {
                    pos = v_in[1] as usize;
                } else {
                    pos += len;
                }
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set pointer to {}", pos);
                }
            }
            Instructions::Jif => {
                len = 3;
                let (v_in, _) = get_values(prog, pos, &modes, 0, 1, 0);
                if v_in[0] == 0 {
                    pos = v_in[1] as usize;
                } else {
                    pos += len;
                }
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set pointer to {}", pos);
                }
            }
            Instructions::Less => {
                len = 4;
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                if v_in[0] < v_in[1] {
                    prog.insert(out, 1);
                } else {
                    prog.insert(out, 0);
                }
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!(
                        "=> set prog[{}] to input ({})",
                        out,
                        prog.get(&out).unwrap()
                    );
                }
                pos += len;
            }
            Instructions::Equ => {
                len = 4;
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                if v_in[0] == v_in[1] {
                    prog.insert(out, 1);
                } else {
                    prog.insert(out, 0);
                }
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!(
                        "=> set prog[{}] to input ({})",
                        out,
                        prog.get(&out).unwrap()
                    );
                }
                pos += len;
            }
            Instructions::End => {
                len = 1;
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> the prog ends");
                }
                return Ok(0);
            }
        }
    }
    Err(3)
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
    Add,  // 1
    Mul,  // 2
    In,   // 3
    Out,  // 4
    Jit,  // 5
    Jif,  // 6
    Less, // 7
    Equ,  // 8
    End,  // 99
}

fn parse_instruction(raw: &isize) -> (HashMap<usize, Mode>, Instructions) {
    let instruction = match raw % 100 {
        1 => Instructions::Add,
        2 => Instructions::Mul,
        3 => Instructions::In,
        4 => Instructions::Out,
        5 => Instructions::Jit,
        6 => Instructions::Jif,
        7 => Instructions::Less,
        8 => Instructions::Equ,
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
            get_answer("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 0),
            Some(43210)
        );
        assert_eq!(
            get_answer(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
                0
            ),
            Some(54321)
        );
        assert_eq!(get_answer("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",0), Some(65210));
        assert_eq!(get_answer(include_str!("../assets/day07_input.txt"),5), Some(273814));
    }
}
