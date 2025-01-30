use get_aoc_input as gai;
use std::error::Error;
use nom::{character::complete::{newline, multispace1, i32}, bytes::complete::tag, IResult, multi::separated_list1};

const USING_TEST_INPUT: bool = false;

#[derive(Debug)]
struct Race {
    dist: usize,
    time: usize,
}
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"Time:      7  15   30
Distance:  9  40  200".into()
        } else {
            gai::load_input(1, 6)?
        }
    };

    let (remains, races) = parse_input(&input).unwrap();
    dbg!(remains, &races);

    let result = races.iter().map(|r| eval_race(r)).fold(1, |acc, v| acc*v);
    println!("Answer is: {result}");
    Ok(())
}

fn eval_race(race: &Race) -> usize {
    let mut out = 0;
    for t in 0..race.time {
        let vel = t; // in m/s
        let time_remaining = race.time - t; // in s
        if vel * time_remaining > race.dist { out += 1; }
    }

    out
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, times) = separated_list1(multispace1, i32)(input)?;

    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distances) = separated_list1(multispace1, i32)(input)?;

    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {time: times[i] as usize, dist: distances[i] as usize});
    }

    Ok((input, races))
}
