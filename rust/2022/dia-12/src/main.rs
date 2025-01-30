const EXEMPLE: bool = true;
const DIA: usize = 12;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Posició {
    x: usize,
    y: usize,
}

impl Posició {
    fn mou(&self, dir: &Direcció) -> Self {
        let mut nova_pos = *self;
        match *dir {
            Direcció::Up => nova_pos.y += 1,
            Direcció::Down => nova_pos.y -= 1,
            Direcció::Left => nova_pos.x -= 1,
            Direcció::Right => nova_pos.x += 1,
            Direcció::UpLeft => {nova_pos.mou(&Direcció::Up); nova_pos.mou(&Direcció::Left);},
            Direcció::UpRight => {nova_pos.mou(&Direcció::Up); nova_pos.mou(&Direcció::Right);},
            Direcció::DownLeft => {nova_pos.mou(&Direcció::Down); nova_pos.mou(&Direcció::Left);},
            Direcció::DownRight => {nova_pos.mou(&Direcció::Down); nova_pos.mou(&Direcció::Right);},
        };
        nova_pos
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Mapa (Vec<Vec<char>>);

impl Mapa {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn possible_neightbor_directions(&self, pos: &Posició, visited_positions: &Vec<Posició>) -> Vec<Direcció> {
        let delta_arr = [-1i32, 0, 1];
        let mut max = 0;
        let mut sortida = Vec::new();

        for dy in delta_arr {
            for dx in delta_arr {
                if dx == 0 && dy == 0 { continue; } // Don't want to stand still
                let new_y = (pos.y as i32 + dy) as usize;
                let new_x = (pos.x as i32 + dx) as usize;
                if visited_positions.contains(&Posició { x: new_x, y: new_y }) {continue;}
                let l = {
                    if new_y < 10000 && new_x < 10000 {
                        self[new_y][new_x]
                    } else {
                        'a'
                    }
                };
                let actual = self[pos.y][pos.x];
                if letter_to_height(actual) + 1 == letter_to_height(l) {
                    sortida.push(pair_to_dir(dx, dy));
                }
            }
        }
        
        sortida
    }
}

impl std::ops::Index<usize> for Mapa {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl std::ops::IndexMut<usize> for Mapa {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, Copy)]
enum Direcció {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}


fn find_shortest_path_length(mapa: &Mapa, actual: &Posició, pos_final: &Posició, cnt: Option<usize>, visited_positions: &mut Vec<Posició>) -> Option<usize> {
    dbg!(&actual);
    if actual != pos_final {
        visited_positions.push(*actual);
        let moviments_possibles = mapa.possible_neightbor_directions(&actual, &visited_positions);
        let mut min = 100000;
        for moviment in moviments_possibles {
            dbg!(&moviment);
            let nova_pos = actual.mou(&moviment);
            if cnt.is_some() {
                let nova_cnt = match moviment {
                    Direcció::UpLeft => cnt.unwrap() + 2,
                    Direcció::UpRight => cnt.unwrap() + 2,
                    Direcció::DownLeft => cnt.unwrap() + 2,
                    Direcció::DownRight => cnt.unwrap() + 2,
                    _ => cnt.unwrap() + 1,
                };
                return find_shortest_path_length(mapa, &nova_pos, pos_final, Some(nova_cnt), visited_positions);
            }
        }
        None // TODO: Figure out how to terminate a branch
    } else {
        cnt
    }

}

fn main() {
    let mut mapa = entrada();
    let start = find_char(&mapa, &'S');
    let end = find_char(&mapa, &'E');
    mapa[start.y][start.x] = 'a';
    mapa[end.y][end.x] = 'z';
    let mut actual = start.clone();
    let mut visited_positions: Vec<Posició> = Vec::new();
    let mut cnt = 0;
    let resposta = find_shortest_path_length(&mapa, &mut actual, &end, Some(cnt), &mut visited_positions);


    println!("{:?}", mapa);
    println!("Res: {}", resposta.unwrap());
}

fn pair_to_dir(dx: i32, dy: i32) -> Direcció {
    match (dx, dy) {
        (-1, -1) => Direcció::UpLeft,
        (0, -1) => Direcció::Up,
        (1, -1) => Direcció::UpRight,
        (-1, 0) => Direcció::Left,
        (1, 0) => Direcció::Right,
        (-1, 1) => Direcció::DownLeft,
        (0, 1) => Direcció::Down,
        (1, 1) => Direcció::DownRight,
        (0, 0) => panic!("Checking against own position"),
        (_, _) => unreachable!(),
    }
}

fn find_char(mapa: &Mapa, c: &char) -> Posició {
    for y in 0..mapa.len() {
        for x in 0..mapa[y].len() {
            if mapa[y][x] == *c {
                return Posició{x, y};
            }
        }
    }
    println!("No s'ha trobat '{c}'.");
    unreachable!();
}

fn entrada() -> Mapa {
    let url = format!("https://adventofcode.com/2022/day/{}/input", DIA);
    let mut text = get_aoc_input::get_input(&url).unwrap();

    if EXEMPLE {
        text = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi".to_string();
    }

    let seq_d_strings: Vec<String> = text.lines().map(|s| s.to_string()).collect();
    let sortida: Vec<Vec<char>> = {
        let mut tmp = Vec::new();
        for string in seq_d_strings {
            let mut chars = string.chars().collect::<Vec<char>>();
            // Insert padding
            chars.insert(0, '@');
            chars.insert(chars.len(), '@');

            tmp.push(chars);
        }
        // Insert padding
        let zeroes = vec!['@'; tmp[0].len()];
        tmp.insert(0, zeroes.clone());
        tmp.insert(tmp.len(), zeroes.clone());

        tmp
    };

    Mapa(sortida)
}

fn letter_to_height(c: char) -> u8 {
    c as u8 - '0' as u8
}
