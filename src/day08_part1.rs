use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

    if let Some(answer) = get_answer(input, 25, 6) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, width: usize, height: usize) -> Option<usize> {
    let pixels = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let size = width * height;
    let nbr = pixels.len() / size;
    let mut layers: Vec<(Vec<_>, usize, usize, usize)> = (0..nbr)
        .map(|i| {
            let p = pixels
                .iter()
                .skip(i * size)
                .take(size)
                .cloned()
                .collect::<Vec<_>>();
            let nbr0 = p.iter().filter(|&n| *n == 0).collect::<Vec<_>>().len();
            let nbr1 = p.iter().filter(|&n| *n == 1).collect::<Vec<_>>().len();
            let nbr2 = p.iter().filter(|&n| *n == 2).collect::<Vec<_>>().len();
            (p, nbr0, nbr1, nbr2)
        })
        .collect();
    layers.sort_by_key(|l| l.1);
    // println!("{:?}", layers);
    Some(layers[0].2 * layers[0].3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt"), 3, 2),
            Some(1)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt"), 25, 6),
            Some(2064)
        );
    }
}
