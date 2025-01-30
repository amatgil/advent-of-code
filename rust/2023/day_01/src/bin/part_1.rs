use std::{fs::File, io::{Read, Write}, error::Error};

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        let inp = File::open("input1.txt");
        if let Ok(mut text) = inp {
            println!("Using cached input...");
            let mut contents = String::new();
            text.read_to_string(&mut contents)?;
            contents
        } else {
            println!("Input not downloading, getting...");
            let text = gai::get_input("https://adventofcode.com/2023/day/1/input")?;
            let mut file = File::create("input1.txt")?;
            file.write_all(text.as_bytes())?;
            text
        }
    };
    println!("{}", input);
    /*
    let input = 
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
*/

    let mut vals: Vec<i32> = vec![];
    for line in input.lines() {
        let a = line.chars()
            .filter(|c| c.is_numeric())
            .next().unwrap();

        let b = line.chars().rev()
            .filter(|c| c.is_numeric())
            .next().unwrap();

        vals.push(
            format!("{}{}", a, b).parse().unwrap()
        );
    }
    println!("{:?}", vals);
    println!("Answer: {}", vals.iter().fold(0, |acc, elem| acc + elem));
    Ok(())
}
