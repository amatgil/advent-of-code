use std::error::Error;
use nom::{bytes::complete::tag, IResult, character::complete::{i32, multispace1, newline}, multi::separated_list1};

const USING_TEST_INPUT: bool = true;

#[derive(Debug)]
struct Card {
    _id: i32,
    winning_nums: Vec<i32>,
    gotten_nums: Vec<i32>,
}

fn parse_id(input: &str) -> IResult<&str, i32> {
  let (input, _) = tag("Card")(input)?;
  let (input, _) = multispace1(input)?;
  let (input, id) = i32(input)?;
  let (input, _) = tag(":")(input)?;
  let (input, _) = multispace1(input)?;
  Ok((input, id))
}

fn parse_nums(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, nums) = separated_list1(multispace1, i32)(input)?;
    Ok((input, nums))
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _id) = parse_id(input)?;
    let (input, winning_nums) = parse_nums(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, gotten_nums) = parse_nums(input)?;
    dbg!(&gotten_nums);
    Ok((input, Card { _id, winning_nums, gotten_nums }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, parse_line)(input)
}

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into()
        } else {
            gai::load_input(1, 4)?
        }
    };

    println!("{:?}", input);
    let (input, cards) = parse_input(&input).unwrap();
    println!("Remaining: '{:?}'", input);

    let fin = cards.into_iter().map(|card| {
        card.gotten_nums.into_iter().fold(0, |acc, num| if card.winning_nums.contains(&num) {acc + 1} else {acc})
    })
        .filter(|v| v != &0)
        .fold(0, |acc, cnt| acc + 2i32.pow(cnt - 1));

    println!("Answer is: {fin}");
    Ok(())
}
