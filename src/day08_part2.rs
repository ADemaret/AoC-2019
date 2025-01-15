use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 2 --");
    let now = Instant::now();

    // let input = "0222112222120000";
    let input = include_str!("../assets/day08_input.txt");

    if let Some(answer) = get_answer(input, 25, 6) {
        //25, 6) {
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
    let layers: Vec<Vec<_>> = (0..nbr)
        .map(|i| pixels.iter().skip(i * size).take(size).cloned().collect())
        .collect();

    let mut images = Vec::new();
    for l in &layers {
        let im: Vec<Vec<_>> = (0..height)
            .map(|i| l.iter().skip(i * width).take(width).cloned().collect())
            .collect();
        images.push(im);
    }
    // println!("{:?}", images);

    for h in 0..height {
        for w in 0..width {
            for img in &images {
                match img[h][w] {
                    0 => {
                        print!(" ");
                        break;
                    }
                    1 => {
                        print!("#");
                        break;
                    }
                    2 => {}
                    _ => panic!(),
                }
            }
        }
        println!()
    }

    None
}
