use std::error::Error;
use std::ops::Index;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1, newline};

const USING_TEST_INPUT: bool = false;

use get_aoc_input as gai;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".into() // Should return 2
        } else {
            gai::load_input(1, 8)?
        }
    };
    let _inputb = 
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"; // Should return 6

    println!("{:?}", input);
    let (remains, map) = parse_input(&input).unwrap();

    let mut label = Node("AAA");
    let mut result = 0;
    for (i, direction) in map.steps.iter().cycle().enumerate() {
        if label.0 == "ZZZ" { result = i; break; }
        match direction {
            Direction::Left => label = map[&label].left.clone(),
            Direction::Right => label = map[&label].right.clone(),
        }
    };
    println!("Answer is: {result}");
    Ok(())
}

impl<'a> Index<&Node<'a>> for Map<'a> {
    type Output = Relationship<'a>;

    fn index(&self, index: &Node<'a>) -> &Self::Output {
        for rel in &self.relationships {
            if rel.source.0 == index.0 { return rel; }
        }
        unreachable!("Node label wasn't in mapping")
    }
}



#[derive(Debug, Clone)]
struct Map<'a> {
    steps: Vec<Direction>,
    relationships: Vec<Relationship<'a>>,
}
#[derive(Debug, Clone)]
enum Direction {
    Left, Right
}
#[derive(Debug, Clone)]
struct Relationship<'a> {
    source: Node<'a>,
    left: Node<'a>,
    right: Node<'a>
}
#[derive(Debug, Clone)]
struct Node<'a>(&'a str);

fn parse_input(input: &str) -> IResult<&str, Map> {
    let (input, steps) = alpha1(input)?;
    let steps: Vec<_> = steps.chars().map(|c| if c == 'L' {Direction::Left} else { Direction::Right }).collect();

    let (input, _) = multispace1(input)?;
    dbg!(input);
    let (input, relationships) = separated_list1(newline, parse_rel)(input)?;
    dbg!(input);


    Ok((input, Map { steps, relationships}))
}

fn parse_rel(input: &str) -> IResult<&str, Relationship> {
    let (input, source) = alpha1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, _) = tag("(")(input)?;

    let (input, (left, right)) = separated_pair(alpha1,tag(", "), alpha1)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Relationship { 
        source: source.into(),
        left: left.into(),
        right: right.into()
    }))
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}
