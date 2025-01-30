use std::error::Error;

const USING_TEST_INPUT: bool = true;

use get_aoc_input as gai;

#[derive(Debug)]
enum Item {
    Nothing,
    Number(usize),
    Symbol,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
struct DeltaPos {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".into()
        } else {
            gai::load_input(1, 3)?
        }
    };

    let input_width = input.lines().next().unwrap().len();
    let input_height = input.len();
    let mut engine_items: Vec<(Item, Option<Vec<Position>>)> = vec![];


    for (y, line) in input.lines().enumerate() {
        let mut n = 0;
        let mut positions = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if n != 0 { engine_items.push((Item::Number(n), Some(positions))); }
                    n = 0; positions = vec![];
                    engine_items.push((Item::Nothing, None));
                },
                '0'..='9' => {
                    n *= 10;
                    n += c as usize - '0' as usize;
                    positions.push(Position { x, y });
                },
                '\n' => panic!("FUCK"),
                _ => {
                    if n != 0 { engine_items.push((Item::Number(n), Some(positions))); }
                    n = 0; positions = vec![];
                    engine_items.push((Item::Symbol, Some(vec![Position { x, y }])));
                }
            }
        }
        if n != 0 { engine_items.push((Item::Number(n), Some(positions.clone()))); }
    }
    dbg!(&engine_items);

    let deltas = [
        DeltaPos {x: -1, y: -1}, DeltaPos {x: -1, y:  0}, DeltaPos {x: -1, y: 1},
        DeltaPos {x:  0, y: -1},                          DeltaPos {x:  0, y: 1},
        DeltaPos {x:  1, y: -1}, DeltaPos {x:  1, y:  0}, DeltaPos {x:  1, y: 1},
    ];

    let mut vals = vec![];

    'engine_loop: for i in 0..engine_items.len() {
        if let (Item::Number(n), positions) = &engine_items[i] {
            let mut positions_to_find = vec![];
            for delta_p in &deltas {
                for pos in positions.as_ref().unwrap() {
                    positions_to_find.push(add_pos(&pos, delta_p, input_width, input_height));
                }
            }
            let positions_to_find: Vec<Position> = positions_to_find.iter().filter_map(|v| *v).collect();
            for p_to_find in &positions_to_find { // We look in engine for a `p_to_find` that's a
                                                  // symbol
                for t in &engine_items {
                    if let (Item::Symbol, Some(positions_checking)) = &t { // We're using the extra
                                                                           // slot in the vec to
                                                                           // store the symbol's pos
                        if positions_checking.contains(&p_to_find) {
                            vals.push(n);
                            continue 'engine_loop;
                        }
                    }
                }
            }
        }
    }
    println!("Vals are: {:?}", vals);
    println!("Sum is: {:?}", vals.into_iter().sum::<usize>());
    Ok(())
}

fn add_pos(s: &Position, rhs: &DeltaPos, w: usize, h: usize) -> Option<Position> {
    let new_x = s.x as i32 + rhs.x;
    if new_x < 0 || new_x >= w as i32 {return None;}

    let new_y = s.y as i32 + rhs.y;
    if new_y < 0 || new_y >= h as i32 {return None;}

    Some(Position { x: new_x as usize, y: new_y as usize})
}
