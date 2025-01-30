use std::{error::Error, fmt::Display, collections::HashMap, u128, ops::Deref};

const USING_TEST_INPUT: bool = true;

use nom::{bytes::complete::{tag, take_while1}, IResult, character::{complete::{i32, multispace1, newline, multispace0, u128}, is_newline, is_alphabetic}, multi::separated_list1, sequence::tuple};
use nom::bytes::complete::take_until1;

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".into()
        } else {
            gai::load_input(1, 5)?
        }
    };

    println!("{:?}", input);
    let (remaining, parsed) = parse_input(&input).unwrap();
    //dbg!(remaining, parsed);
    
    // The relationships are a 2D table with the following shape:
    // | Seed | Soil | Fertil | Water | ...   (There are eight columns total)
    // | 98   |  50  | ..
    // | 99   |  51  | ..
    for mapping in &parsed.1 {
        println!("-'{}' - '{}' => '{:?}'", mapping.source, mapping.dest, mapping.range);
    }
    let (obj_seeds, almanac) = parsed;
    //let mut relationships: Vec<Relationship> = vec![];
    //let relationships: Vec<(&str, HashMap<i32, i32>)> = vec![];
    //for seed in &obj_seeds { relationships.push(Relationship::from(*seed)); }

    let mut historical_values = vec![];
    let mut values = obj_seeds.clone();

    historical_values.push((idx_to_word(0), values.clone()));
    for attr_idx in 1..8 {
        for v in &mut values {
            for mapping in &almanac {
                if mapping.source == idx_to_word(attr_idx) {
                    for rang in &mapping.range {
                        if (rang.source_start..rang.source_start + rang.len).contains(&v) {
                            dbg!(&v, rang);
                            *v += (*v as i128 + rang.dest_start as i128 - rang.source_start as i128) as u128 ; // v - source + dest
                        }
                    }
                }
            }

        }
        historical_values.push((idx_to_word(attr_idx), values.clone()));
    }

    println!("Final values are: {:?}", values);
    /*
    for seed in &obj_seeds {
        for mapping in &almanac {
            let Map { source, dest, range } = &mapping;
            for rang in range {
                for relationship in &mut relationships {
                    if let Some(seed_val) = relationship.get("seed") {
                        if seed == &seed_val {
                            if let Some(ptr) = relationship.get_mut(source) {
                                if (rang.source_start..rang.source_start+rang.len).contains(ptr) {
                                    let val = rang.dest_start - rang.source_start;
                                    relationship.update_chain(dest, val);
                                }
                            }
                            for n in rang.source_start..rang.source_start+rang.len {
                                *relationship
                                    .get_mut(dest)
                                    .expect("Should exist after initializing") = n;
                            }
                        }
                    }
                }
            }

        }
    }
    */
    Ok(())
}

fn word_to_idx(s: &str) -> usize {
     match s {
        "seed" => 0,
        "soil" => 1,
        "fertilizer" => 2,
        "water" => 3,
        "light" => 4,
        "temperature" => 5,
        "humidity" => 6,
        "location" => 7,
        _ => panic!("Tried to update chain at key: {s}"),
    }
}
fn idx_to_word(v: u128) -> &'static str {
     match v {
        0 => "seed",
        1 => "soil" ,
        2 => "fertilizer" ,
        3 => "water",
        4 => "light",
        5 => "temperature",
        6 => "humidity",
        7 => "location",
        _ => panic!("Tried to update chain at key: {v}"),
    }
}

#[derive(Debug)]
struct Relationship (
    //seed: u128, soil: u128, fertilizer: u128,
    //water: u128, light: u128, temperature: u128,
    //humidity: u128, location: u128,
    [u128; 8]
);

impl Relationship {
    fn from(v: u128) -> Self {
        Self([v; 8])
        //Self { seed: v, soil: v, fertilizer: v, water: v,
        //    light: v, temperature: v, humidity: v, location: v,
        //}
    }
    fn get(&self, key: &str) -> Option<u128> {
        match key {
            "seed" => Some(self.0[0]),
            "soil" => Some(self.0[1]),
            "fertilizer" => Some(self.0[2]),
            "water" => Some(self.0[3]),
            "light" => Some(self.0[4]),
            "temperature" => Some(self.0[5]),
            "humidity" => Some(self.0[6]),
            "location" => Some(self.0[7]),
            _ => None,
        }
    }
    fn update_chain(&mut self, key: &str, val: u128) {
         let idx = match key {
            "seed" => 0,
            "soil" => 1,
            "fertilizer" => 2,
            "water" => 3,
            "light" => 4,
            "temperature" => 5,
            "humidity" => 6,
            "location" => 7,
            _ => panic!("Tried to update chain at key: {key}"),
        };
        for i in idx..8 {
            self.0[i] = val;
        }

    }
    fn get_mut(&mut self, key: &str) -> Option<&mut u128> {
        match key {
            "seed" => Some(&mut self.0[0]),
            "soil" => Some(&mut self.0[1]),
            "fertilizer" => Some(&mut self.0[2]),
            "water" => Some(&mut self.0[3]),
            "light" => Some(&mut self.0[4]),
            "temperature" => Some(&mut self.0[5]),
            "humidity" => Some(&mut self.0[6]),
            "location" => Some(&mut self.0[7]),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    dest_start: u128,
    source_start: u128,
    len: u128,
}


#[derive(Debug, Clone)]
struct Map<'a> {
    source: &'a str,
    dest: &'a str,
    range: Vec<MapRange>,
}
impl<'a> Display for Map<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- S: '{}'; D: '{}': {:?}", self.source, self.dest, self.range)
    }
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<MapRange>> {
    separated_list1(tag("\n"), parse_range)(input)
}

fn parse_range(input: &str) -> IResult<&str, MapRange> {
    let (input, (dest_range_start, _, source_range_start, _, range_len)) = tuple((u128, multispace1, u128, multispace1, u128))(input)?;
    Ok((input, MapRange {
        dest_start: dest_range_start,
        source_start: source_range_start,
        len: range_len
    }))

}
fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = multispace0(input)?;
    let (input, source_str) = take_until1("-")(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, dest_str) = take_until1(" ")(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, ranges) = parse_ranges(input)?;
    let map = Map {
        dest: dest_str,
        source: source_str,
        range: ranges
    };
    //let (input, _) = multispace1(input)?;
    //dbg!(map, input);
    Ok((input, map))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(tag("\n"), parse_map)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u128>, Vec<Map>)> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, objective_seeds) = separated_list1(multispace1, u128)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, maps) = parse_maps(input)?;

    Ok((input, (objective_seeds, maps)))
}
