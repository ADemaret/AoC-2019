use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const VERBOSE: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

    if let Some(answer) = get_answer(input, 5) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
fn get_answer(input_file: &str, input_value: isize) -> Option<isize> {
    let mut prog = get_input(input_file);
    let mut input = input_value;

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
                pos += len;
            }
            Instructions::Mul => {
                let (v_in, out) = get_values(prog, pos, &modes, 0, 1, 2);
                prog.insert(out, v_in[0] * v_in[1]);
                if VERBOSE {
                    print_instruction(prog, pos, instruction, len);
                    println!("=> set prog[{}] to {}", out, prog.get(&out).unwrap());
                }
                pos += len;
            }
            Instructions::In => {
                len = 2;
                let out = *prog.get(&(pos + 1)).unwrap() as usize;
                prog.insert(out, *input);
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
                break;
            }
        }
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
        assert_eq!(get_answer("3,9,8,9,10,9,4,9,99,-1,8", 7), Some(0));
        assert_eq!(get_answer("3,9,8,9,10,9,4,9,99,-1,8", 8), Some(1));
        assert_eq!(get_answer("3,9,8,9,10,9,4,9,99,-1,8", 9), Some(0));

        assert_eq!(get_answer("3,9,7,9,10,9,4,9,99,-1,8", 7), Some(1));
        assert_eq!(get_answer("3,9,7,9,10,9,4,9,99,-1,8", 8), Some(0));
        assert_eq!(get_answer("3,9,7,9,10,9,4,9,99,-1,8", 9), Some(0));

        assert_eq!(get_answer("3,3,1108,-1,8,3,4,3,99", 7), Some(0));
        assert_eq!(get_answer("3,3,1108,-1,8,3,4,3,99", 8), Some(1));
        assert_eq!(get_answer("3,3,1108,-1,8,3,4,3,99", 9), Some(0));

        assert_eq!(get_answer("3,3,1107,-1,8,3,4,3,99", 7), Some(1));
        assert_eq!(get_answer("3,3,1107,-1,8,3,4,3,99", 8), Some(0));
        assert_eq!(get_answer("3,3,1107,-1,8,3,4,3,99", 9), Some(0));

        assert_eq!(
            get_answer("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0),
            Some(0)
        );
        assert_eq!(
            get_answer("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1),
            Some(1)
        );

        assert_eq!(
            get_answer("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0),
            Some(0)
        );
        assert_eq!(
            get_answer("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1),
            Some(1)
        );

        assert_eq!(get_answer("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",7),Some(999));
        assert_eq!(get_answer("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",8),Some(1000));
        assert_eq!(get_answer("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",9),Some(1001));

        assert_eq!(
            get_answer(include_str!("../assets/day05_input.txt"), 5),
            Some(7873292)
        );
    }
}
