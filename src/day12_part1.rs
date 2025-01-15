use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    // let input = include_str!("../assets/day12_input_demo2.txt");
    let input = include_str!("../assets/day12_input.txt");

    if let Some(answer) = get_answer(input, 1000) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Clone)]
struct Moon {
    id: usize,
    x: isize, // position
    y: isize,
    z: isize,
    vx: isize, // velocity
    vy: isize,
    vz: isize,
}

impl Moon {
    pub fn gravity(&mut self, v_xyz: &Vec<Moon>) {
        for v in v_xyz {
            if v.id != self.id {
                self.vx = match v.x.cmp(&self.x) {
                    std::cmp::Ordering::Greater => self.vx + 1,
                    std::cmp::Ordering::Less => self.vx - 1,
                    std::cmp::Ordering::Equal => self.vx,
                };
                self.vy = match v.y.cmp(&self.y) {
                    std::cmp::Ordering::Greater => self.vy + 1,
                    std::cmp::Ordering::Less => self.vy - 1,
                    std::cmp::Ordering::Equal => self.vy,
                };
                self.vz = match v.z.cmp(&self.z) {
                    std::cmp::Ordering::Greater => self.vz + 1,
                    std::cmp::Ordering::Less => self.vz - 1,
                    std::cmp::Ordering::Equal => self.vz,
                };
            }
        }
    }

    pub fn velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }
}

fn get_answer(input: &str, steps: usize) -> Option<isize> {
    let mut v_coords = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let xyz = line
                .split(['=', ',', '>'])
                .filter_map(|v| v.parse::<isize>().ok())
                .collect::<Vec<_>>();
            Moon {
                id: i + 1,
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
                vx: 0,
                vy: 0,
                vz: 0,
            }
        })
        .collect::<Vec<Moon>>();

    // println!("After 0 steps:");
    // for v in v_coords.iter_mut() {
    //     println!(
    //         "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:2}, y={:2}, z={:2}>",
    //         v.x, v.y, v.z, v.vx, v.vy, v.vz
    //     );
    // }
    // println!();

    for _time in 1..=steps {
        let org_vec = v_coords.clone();
        // println!("After {time} steps:");
        for v in v_coords.iter_mut() {
            v.gravity(&org_vec);
            v.velocity();
            // println!(
            //     "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:2}, y={:2}, z={:2}>",
            //     v.x, v.y, v.z, v.vx, v.vy, v.vz
            // );
        }
        // println!();
    }

    let mut sum = 0;
    for v in v_coords {
        sum += (v.x.abs() + v.y.abs() + v.z.abs()) * (v.vx.abs() + v.vy.abs() + v.vz.abs());
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt"), 10),
            Some(179)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo2.txt"), 100),
            Some(1940)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt"), 1000),
            Some(12490)
        );
    }
}
