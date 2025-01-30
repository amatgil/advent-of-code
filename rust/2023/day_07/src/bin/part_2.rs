use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use get_aoc_input as gai;
use nom::{IResult, bytes::complete::take_while1, character::complete::{newline, multispace1, i32}, multi::separated_list1};

const USING_TEST_INPUT: bool = false;
const CARD_LIST: &str = "AKQT98765432J";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Play<'a> {
    hand: Hand<'a>,
    bid: usize,
    rank: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<'a>(&'a str);

impl<'a> Ord for Play<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let score_self = eval_hand(&self.hand);
        let score_other = eval_hand(&other.hand);

        let base_cmp = score_self.cmp(&score_other);
        if base_cmp != Ordering::Equal { return base_cmp; }
        self.hand.partial_cmp(&other.hand).unwrap()
    }
}
impl<'a> Ord for Hand<'a> {
    /// Note that both are always len 5
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { 
        let mut c_chars = self.0.chars();
        let mut d_chars = other.0.chars();
        for _ in 0..self.0.len() {
            let c = c_chars.next().unwrap();
            let d = d_chars.next().unwrap();
            if card_worth(c) > card_worth(d) { return Ordering::Greater; }
            else if card_worth(c) < card_worth(d) { return Ordering::Less; }
        }
        return Ordering::Equal;
    }
}

impl<'a> PartialOrd for Play<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum HandValue {
    FiveOfAKind   = 7,  // AAAAA
    FourOfAKind   = 6,  // AAAAB
    FullHouse     = 5,  // AAABB
    ThreeOfAKind  = 4,  // AAABC
    TwoPair       = 3,  // AABBD
    OnePair       = 2,  // AABCD
    HighCard      = 1,  // ABCDE
}

impl HandValue {
    fn from_val(v: usize) -> Self {
        match v {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => Self::ThreeOfAKind,
            2 => Self::TwoPair,
            1 => Self::HighCard,
            _ => panic!("Unrecognized value"),
        }
    }
}

fn apply_rankings(plays: &mut Vec<Play>) {
    plays.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, play) in plays.iter_mut().enumerate() {
        play.rank = Some(i + 1)
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".into()
        } else {
            gai::load_input(1, 7)?
        }
    };

    let (remaining, mut plays) = parse_input(&input).unwrap();
    dbg!(remaining, &plays);
    apply_rankings(&mut plays);

    for play in &plays{ println!("{}",play); }
    let result: usize = plays.into_iter().fold(0, |acc, pl| acc + pl.bid * pl.rank.unwrap());

    println!("Answer is: {}", result);
    Ok(())
}


fn eval_hand(in_hand: &Hand) -> HandValue {
    let mut max_value = HandValue::HighCard; // minimum value

    for new_joker_val in CARD_LIST.chars() {
        // Change 'J' for the one we're trying
        let mut hand = in_hand.clone();
        let new_hand_labels = hand.0.replace('J', &new_joker_val.to_string());
        hand.0 = &new_hand_labels;

        // Check as in Part 1
        let mut matches: Vec<usize> = Vec::new();
        for c in hand.0.chars() { matches.push(hand.0.chars().filter(|&x| x == c).count()); }

        let max = *matches.iter().max().unwrap();
        if max == 5 || max == 4 || max == 1 { max_value = std::cmp::max(HandValue::from_val(max), max_value);}
        else {
            if max == 3 {
                if matches.contains(&2) { max_value = std::cmp::max(HandValue::FullHouse, max_value); }
                else { max_value = std::cmp::max(HandValue::ThreeOfAKind, max_value); }
            } else { // max == 2, need to check if it's (2,2,1) or (2,1,1)
                let number_of_twos = matches.iter()
                    .filter(|&&v| v == 2).count() / 2; // Div two because duped
                if number_of_twos == 1 { max_value = std::cmp::max(HandValue::OnePair, max_value); }
                else { max_value = std::cmp::max(HandValue::TwoPair, max_value); }
            }
        }
    }

    max_value

}

fn parse_play(input: &str) -> IResult<&str, Play> {
    let (input, hand) = take_while1(is_alphanum)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, bid) = i32(input)?;

    Ok((input, Play { hand: Hand(hand), bid: bid as usize, rank: None }))
}

fn parse_input(input:&str) -> IResult<&str, Vec<Play>> { separated_list1(newline, parse_play)(input) }

fn card_worth(label: char) -> usize {
    CARD_LIST.len() -  CARD_LIST.find(label).unwrap()
}


fn is_alphanum(c: char) -> bool {
    (b'A'..=b'Z').contains(&(c as u8)) ||
        (b'0'..=b'9').contains(&(c as u8))
}

impl<'a> Display for Play<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "H: {}  B: {}  (R: {})", self.hand.0, self.bid, self.rank.unwrap())
    }
}

#[test]
pub fn test_evaluator() {
    use std::collections::HashMap;

    let pairs = HashMap::from([
        (HandValue::FiveOfAKind,  "AAAAA"), (HandValue::FourOfAKind,  "AAAAB"),
        (HandValue::FullHouse,    "AAABB"), (HandValue::ThreeOfAKind, "AAABC"),
        (HandValue::TwoPair,      "AABBD"), (HandValue::OnePair,      "AABCD"),
        (HandValue::HighCard,     "ABCDE"), (HandValue::OnePair,      "32T3K"),
        (HandValue::TwoPair,      "KK677"), (HandValue::FourOfAKind,  "KTJJT"),
        (HandValue::FourOfAKind,  "T55J5"), (HandValue::FourOfAKind,  "QQQJA"),
        (HandValue::FullHouse,    "JAABB"), (HandValue::ThreeOfAKind, "JAABC"),
    ]);

    for (val, hand) in pairs.iter() {
        println!("Evaling {}, expecting: {:?}", hand, val);
        assert_eq!(*val, eval_hand(&Hand(hand)))
    }
}
