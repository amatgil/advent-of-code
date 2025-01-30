use std::fs;

const EXEMPLE: bool = true;
const DIA: usize = 11;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    index: usize,
    items: Vec<usize>,
    operation: (Operator, Option<usize>),
    test_number: usize,
    when_true: usize,
    when_false: usize,
    items_inspected: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Square,
}

impl Operator {
    fn from_char(c: &str) -> Self {
        match c {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Times,
            "/" => Self::Divide,
            _ => unreachable!("Wack ass operator: {c}"),
        }
    }
}

fn part_2() {
    let entrada = entrada_to_vec();
    let mut monkeys = entrada_to_micos(entrada);
}
fn main() {
    let entrada = entrada_to_vec();
    let mut monkeys = entrada_to_micos(entrada);

    for _round in 0..10000 {
        println!("Monkeys in round {_round} are holding:");
        for monke in monkeys.clone() {
            println!("{}: {:#?}", monke.index, monke.items);
        }

        for i in 0..monkeys.len() {
            let monke = monkeys[i].clone();
            for item in monke.items {
                let new_item_value = get_new_item_value(item , monke.operation ); // Inspection
                monkeys[i].items_inspected += 1;
                let passes_test = new_item_value % monke.test_number == 0;
                if passes_test {
                    monkeys[monke.when_true].items.push(new_item_value);
                } else {
                    monkeys[monke.when_false].items.push(new_item_value);
                }

            }
            monkeys[i].items = Vec::new(); // It threw away all its items, it holds nothing
        }

    }

    let monkey_business = {
        let mut tmp = Vec::new();
        for monke in monkeys {
            tmp.push(monke.items_inspected);
        }
        dbg!(&tmp);
        tmp.sort();
        tmp.reverse();
        tmp[0] * tmp[1]
    };

    println!("Resposta: {monkey_business}");

}


fn get_new_item_value(item: usize, op: (Operator, Option<usize>)) -> usize {
    match op.0 {
        Operator::Plus => item + op.1.unwrap(),
        Operator::Minus => item - op.1.unwrap(),
        Operator::Times => item * op.1.unwrap(),
        Operator::Divide => item / op.1.unwrap(),
        Operator::Square => item * item,
    }
}
fn entrada_to_micos(e: Vec<String>) -> Vec<Monkey> {
    let mut sortida = Vec::new();
    for monke in e.chunks(7) {
        let index = monke[0].replace(":", "").split(' ').nth(1).unwrap().parse().unwrap();
        let items: Vec<usize> = monke[1].replace(",", "").split(' ').skip(4).map(|s| s.parse().unwrap()).collect();
        let operation = get_operation(&monke[2]);
        let test_number = monke[3].split(' ').nth(5).unwrap().parse::<usize>().unwrap();
        let when_true = monke[4].split(' ').nth(9).unwrap().parse::<usize>().unwrap();
        let when_false = monke[5].split(' ').nth(9).unwrap().parse::<usize>().unwrap();

        sortida.push(Monkey{index, items, operation, test_number, when_true, when_false, items_inspected: 0 });

    }
    sortida
}


fn get_operation(s: &str) -> (Operator, Option<usize>) {
    let op_op = Operator::from_char(s.split(' ').nth(6).unwrap());
    let n: Option<usize> = str_to_n(s.split(' ').nth(7).unwrap());
    let operation = (op_op, n);

    if operation.0 == Operator::Times && operation.1.is_none() {
        return (Operator::Square, None);
    } 

    operation

}

fn str_to_n(s: &str) -> Option<usize> {
    if let Ok(n) = s.parse::<usize>()  {
        Some(n)
    } else {
        None
    }
}
    
fn entrada_to_vec() -> Vec<String> {
    let url = format!("https://adventofcode.com/2022/day/{}/input", DIA);
    let mut text = get_aoc_input::get_input(&url).unwrap();

    if EXEMPLE {
        text = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1".to_string()
    }

    let seq_d_strings: Vec<String> = text.lines().map(|s| s.to_string()).collect();

    seq_d_strings
}
