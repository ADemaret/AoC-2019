use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 2 --");
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
            let mut m = d.parse::<isize>().unwrap();
            let mut fuel = 0;
            mass_to_fuel(&mut m,&mut fuel);
            fuel
        })
        .sum()
}

fn mass_to_fuel(mass: &mut isize,fuel:&mut usize) {
    *mass = (*mass / 3) - 2;
    if *mass <= 0{
        return;
    }
    *fuel += *mass as usize;
    mass_to_fuel(mass, fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day01_input_demo1.txt")),
            51316
        );
        assert_eq!(
            get_answer(include_str!("../assets/day01_input.txt")),
            4866824
        );
    }
}
