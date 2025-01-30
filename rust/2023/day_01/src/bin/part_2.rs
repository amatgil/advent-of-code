use std::{fs::File, io::{Read, Write}, error::Error, collections::HashMap};

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        let inp = File::open("input2.txt");
        if let Ok(mut text) = inp {
            println!("Using cached input...");
            let mut contents = String::new();
            text.read_to_string(&mut contents)?;
            contents
        } else {
            println!("Input not downloading, getting...");
            let text = gai::get_input("https://adventofcode.com/2023/day/1/input")?;
            let mut file = File::create("input2.txt")?;
            file.write_all(text.as_bytes())?;
            text
        }
    };

    /*
    let input =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
*/

    println!("{:?}", input);
    let mut vals: Vec<i32> = vec![];
    for line in input.lines() {
        let forward = get_first_num(line, false)?;
        let backward = get_first_num(line, true)?;
        vals.push(
            format!("{}{}", forward, backward).parse().unwrap()
        );
    }
    println!("         {:?}", vals);
    println!("Intended: 29, 83, 13, 24, 42, 14, 76.");
    println!("Answer: {}", vals.iter().fold(0, |acc, elem| acc + elem));
    Ok(())
}

fn get_first_num(s: &str, reversed: bool) -> Result<u8, Box<dyn Error>> {
    let numbers = if reversed {
        HashMap::from([
           ("eno", 1), ("owt", 2), ("eerht", 3),
           ("ruof", 4), ("evif", 5), ("xis", 6),
           ("neves", 7), ("thgie", 8), ("enin", 9),
        ])
    } else {
        HashMap::from([
           ("one", 1), ("two", 2), ("three", 3),
           ("four", 4), ("five", 5), ("six", 6),
           ("seven", 7), ("eight", 8), ("nine", 9),
        ])
    };

    let s = if reversed { s.chars().rev().collect::<String>() }
        else { s.to_string() };

    let mut val = 10;

    let mut earliest_idx = 10000;

    for c in s.chars() {
        if c.is_numeric() {
            val = c as u8 - '0' as u8;
            earliest_idx = s.find(c).ok_or("Fuck")?;
            break;
        } 
    }
    for str_nums in numbers.clone().into_keys() {
        if let Some(i) = s.find(str_nums) {
            if earliest_idx > i {
                val = numbers[str_nums];
                earliest_idx = i;
            }
        }
    }
    println!("{s} -> {val} ({reversed})");
    return Ok(val);
}
