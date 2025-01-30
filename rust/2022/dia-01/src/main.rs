use std::{fs, num::ParseIntError, error::Error};

fn main() {
    let mut elves = entrada_to_vec();

    let max = elves.clone().into_iter().max().unwrap(); // Part 1 answer
    println!("Part 1: {}", max);
    elves.sort();
    elves.reverse();
    println!("Part 2: {:?}", elves[0] + elves[1] + elves[2]); // Part 2 answer
}

fn entrada_to_vec() -> Vec<u128> {
    let entrada = get_aoc_input::get_input("https://adventofcode.com/2022/day/1/input").unwrap();
    let seq_d_strings: Vec<&str> = entrada.split('\n').collect();
    let mut elves = Vec::new();

    let mut acc = 0;
    for string in seq_d_strings {
        if string.is_empty() {
            elves.push(acc);
            acc = 0;
        } else {
            acc += string.parse::<u128>().unwrap();
        }
    }
    elves.push(acc);

    elves
}
