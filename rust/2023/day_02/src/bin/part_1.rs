use std::error::Error;

const USING_TEST_INPUT: bool = false;

use get_aoc_input as gai;

#[derive(Debug)]
struct Game {
    id: usize,
    throws: Vec<Throw>
}
#[derive(Debug)]
struct Throw {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into()
        } else {
            gai::load_input(1, 2)?
        }
    };

    let mut games = vec![];
    for line in input.lines() {
        let semicolon = line.find(":").unwrap();
        let id: usize = line[5..semicolon].parse().unwrap();
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
        games.push(Game { id, throws: final_throws });
    }

    let mut valid_ids = vec![];
    'game_loop: for game in games {
        for throw in game.throws {
            if throw.red > 12 || throw.green > 13 || throw.blue > 14 {continue 'game_loop;}
        }
        valid_ids.push(game.id);
    }

    println!("Valid ids: {:?}", valid_ids);
    println!("Answer: {}", valid_ids.iter().sum::<usize>());

    Ok(())
}
