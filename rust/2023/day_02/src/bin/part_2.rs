use std::{error::Error, cmp::max};

const USING_TEST_INPUT: bool = false;

struct Game {
    _id: usize,
    throws: Vec<Throw>
}
#[derive(Debug)]
struct Throw {
    red: usize,
    green: usize,
    blue: usize,
}

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into()
        } else {
            gai::load_input(2, 2)?
        }
    };

    let mut games = vec![];
    for line in input.lines() {
        let semicolon = line.find(":").unwrap();
        let _id: usize = line[5..semicolon].parse().unwrap();
        let throws = &line[semicolon + 1..];
        let mut final_throws = vec![];

        for throw in throws.split(';') {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for dice in throw.split(",") {
                let dice = dice.trim();
                if let Some(i) = dice.find("red"){ red = dice[..i - 1].parse().unwrap(); }
                if let Some(i) = dice.find("green"){ green = dice[..i - 1].parse().unwrap(); }
                if let Some(i) = dice.find("blue"){ blue = dice[..i - 1].parse().unwrap(); }
            }
            final_throws.push(Throw { red, green, blue })
        }

        games.push(Game { _id, throws: final_throws });
    }

    let mut min_cubes_power = vec![];
    for game in games {
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for throw in game.throws {
            min_red = max(throw.red, min_red);
            min_green = max(throw.green, min_green);
            min_blue = max(throw.blue, min_blue);
        }
        min_cubes_power.push(min_red*min_green*min_blue);
    }

    println!("Valid ids: {:?}", min_cubes_power);
    println!("Answer: {}", min_cubes_power.iter().sum::<usize>());

    Ok(())
}
