use std::{collections::HashSet, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo5.txt");
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
    let mut station = (0, 0);
    for l in 0..grid.max_l {
        for c in 0..grid.max_c {
            if asters.contains(&(l, c)) {
                let mut lines = HashSet::new();
                // print!("at {l},{c} :");
                for a in &asters {
                    if *a != (l, c) {
                        let angle = get_angle((l, c), *a);
                        lines.insert((angle * 100_000.0).round() as isize);
                    }
                }
                if lines.len() > max {
                    max = lines.len();
                    station = (l, c);
                }
            }
        }
    }
    // println!("choosen location is {:?}", station);

    // at location
    let mut lines = Vec::new();
    for a in &asters {
        if station != *a {
            let angle = get_angle(station, *a);
            // manhattan dist is ok
            let dist = station.1.abs_diff(a.1) + station.0.abs_diff(a.0);

            lines.push(((angle * 1000.0).round() as usize, dist, a));
        }
    }
    lines.sort();

    let mut sol = None;
    let mut prev_angle = 360;
    let mut count = 1;
    let mut done = Vec::new();
    for l in lines.iter().cycle() {
        if (l.0 != prev_angle && !done.contains(&l.2)) || done.len() >= asters.len() - 2 {
            // println!("{count} : aster {:?} is killed",l);
            count += 1;
            prev_angle = l.0;
            done.push(l.2);
            sol = Some(l.2 .1 * 100 + l.2 .0);
        }
        if done.len() == asters.len() + 1 || count > 200 {
            break;
        }
    }
    sol
}

fn get_angle(station: (usize, usize), aster: (usize, usize)) -> f32 {
    (360.0 + 360.0 - 270.0 - math::get_angle(station, aster)) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        assert_eq!(get_angle((1, 1), (0, 1)), 0.0);
        assert_eq!(get_angle((1, 1), (0, 2)), 45.0);
        assert_eq!(get_angle((1, 1), (1, 2)), 90.0);
        assert_eq!(get_angle((1, 1), (2, 2)), 135.0);
        assert_eq!(get_angle((1, 1), (2, 1)), 180.0);
        assert_eq!(get_angle((1, 1), (2, 0)), 225.0);
        assert_eq!(get_angle((1, 1), (1, 0)), 270.0);
        assert_eq!(get_angle((1, 1), (0, 0)), 315.0);
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo5.txt")),
            Some(802)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input.txt")),
            Some(512)
        );
    }
}
