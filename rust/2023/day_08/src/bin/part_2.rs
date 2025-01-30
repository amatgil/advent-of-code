use std::error::Error;
use std::ops::Index;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1, newline, alphanumeric1};

const USING_TEST_INPUT: bool = false;

use get_aoc_input as gai;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".into()
        } else {
            gai::load_input(1, 8)?
        }
    };

    let (_remains, map) = parse_input(&input).unwrap();
    let starting_nodes: Vec<_> = map.relationships.iter()
        .filter(|&node| node_ends_in(&node.source, 'A')).collect();

    let mut results: Vec<usize> = Vec::new();
    for start in starting_nodes {
        let mut label: &Node = &start.source;
        for (i, direction) in map.steps.iter().cycle().enumerate() {
            if node_ends_in(&label, 'Z') { results.push(i); break; }
            match direction {
                Direction::Left => label = &map[&label].left,
                Direction::Right => label = &map[&label].right,
            }
        };
    }

    // mcm being associative is SO useful omg
    let result: usize = results.iter().fold(1, |acc, &b| mcm(acc, b));

    println!("Answer is: {result}");
    Ok(())
}

fn node_ends_in(node: &Node, c: char) -> bool {
    node.0.chars().last().unwrap() == c
}
fn mcd(mut a: usize, mut b:usize) -> usize{
    if a == b { return a; }
    if b > a { std::mem::swap(&mut a, &mut b); }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn mcm(a: usize, b: usize) -> usize {
    a * b / mcd(a, b)
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
    let (input, relationships) = separated_list1(newline, parse_rel)(input)?;


    Ok((input, Map { steps, relationships}))
}

fn parse_rel(input: &str) -> IResult<&str, Relationship> {
    let (input, source) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, _) = tag("(")(input)?;

    let (input, (left, right)) = separated_pair(alphanumeric1,tag(", "), alphanumeric1)(input)?;
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
