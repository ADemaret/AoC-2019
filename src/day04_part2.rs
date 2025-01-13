use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

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
    let (min_val, max_val) = input
        .split_once("-")
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())).unwrap();

    let (min, max) = input
        .split_once("-")
        .map(|(a, b)| {
            (
                a.chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect::<Vec<_>>(),
                b.chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    let mut ok = 0;
    for ch0 in min[0]..=max[0] {
        for ch1 in 0..=9 {
            if ch1 >= ch0 {
                for ch2 in 0..=9 {
                    if ch2 >= ch1 {
                        for ch3 in 0..=9 {
                            if ch3 >= ch2 {
                                for ch4 in 0..=9 {
                                    if ch4 >= ch3 {
                                        for ch5 in 0..=9 {
                                            if ch5 >= ch4 && ((ch0 == ch1 && ch1 != ch2)
                                                || (ch0 != ch1 && ch1 == ch2 && ch2 != ch3)
                                                || (ch1 != ch2 && ch2 == ch3 && ch3 != ch4)
                                                || (ch2 != ch3 && ch3 == ch4 && ch4 != ch5)
                                                || (ch3 != ch4 && ch4 == ch5))
                                            {
                                                let val = ch0 * 100000
                                                    + ch1 * 10000
                                                    + ch2 * 1000
                                                    + ch3 * 100
                                                    + ch4 * 10
                                                    + ch5;
                                                if val >= min_val && val <= max_val {
                                                    // println!("{val}");
                                                    ok += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("112233-112233"), Some(1));
        assert_eq!(get_answer("123444-123444"), Some(0));
        assert_eq!(get_answer("111122-111122"), Some(1));
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), Some(617));
    }
}
