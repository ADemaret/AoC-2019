use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo2.txt");
    let input = include_str!("../assets/day06_input.txt");

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
    let mut links = HashMap::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once(")").unwrap();
        let mut entry = links.entry(a).or_insert(Vec::new());
        entry.push(b);
        entry = links.entry(b).or_insert(Vec::new());
        entry.push(a);
    });

    // for l in &links {
    //     println!("{} {:?}", l.0, l.1);
    // }

    bfs(&links, "YOU", "SAN")
}

fn bfs(graph: &HashMap<&str, Vec<&str>>, start_node: &str, end_node: &str) -> Option<usize> {
    let mut dejavu = Vec::new();
    let mut queue = VecDeque::new();

    dejavu.push(start_node);
    queue.push_back((start_node, 0)); // first cost is 0
    while let Some((current_node, cost)) = queue.pop_front() {
        // found
        if current_node == end_node {
            return Some(cost - 2);
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for link in graph.get(&current_node).unwrap() {
            if !dejavu.contains(link) {
                dejavu.push(link);
                queue.push_back((link, cost + 1));
            }
        }
    }
    // not found
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo2.txt")),
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            Some(379)
        );
    }
}
