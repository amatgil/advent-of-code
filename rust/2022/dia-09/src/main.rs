use std::fs;

const EXEMPLE: bool = true;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Posició {
    x: isize,
    y: isize,
}

fn main() {
    let e = entrada(); let entrada = split_entrada(&e);
    let start = Posició {x: 11, y: 5}; 
    let mut rope = [start; 10];
    let mut visited_positions: Vec<Posició> = Vec::new();

    for linia in entrada {
        let (dir, quant): (char, usize) = (linia[0].chars().collect::<Vec<char>>()[0], linia[1].parse().unwrap());

        for i in 0..quant {
            move_head(&mut rope[0], &dir);
            if i != quant-1 {
                for t in 1..=9 {
                    move_tail(&rope[0].clone(), &mut rope[t]);
                }
            }
            //println!("h: ({}, {}); t: ({}, {})", head.x as isize - start.x as isize, head.y as isize - start.y as isize, tail.x as isize - start.x as isize, tail.y as isize - start.y as isize);
            if !visited_positions.contains(&rope[9]) {
                visited_positions.push(rope[9]);
                //println!("({}, {}) wasn't visited yet!", tail.x, tail.y);
            }
            print_grid(&rope);
        }
    }

    print_grid(&rope);
    dbg!(&rope);

    println!("Resposta de la part 1: {}", visited_positions.len());

}

fn print_grid(rope: &[Posició]) {
    const x_size: usize = 27;
    const y_size: usize = 22;
    let mut grid = [["· "; y_size]; x_size];


    for bit in rope {
        grid[bit.x as usize][bit.y as usize] = "B ";
    }

    for y in 0..y_size-1 {
        for x in 0..x_size-1 {
            print!("{}", grid[x][y]);
        }
        println!();
    }

    println!("-------------");
}

fn move_tail(head: &Posició, tail: &mut Posició) {
    if head == tail {
        return ();
    }
    let moviment = find_tail_move(head, tail);
    match moviment.0 {
       // Top left: -+
       // Bottom right: +-
       'U' => tail.y += 1,
       'D' => tail.y -= 1,
       'L' => tail.x -= 1,
       'R' => tail.x += 1,
       _ => panic!("???? TF??"),
    }
    match moviment.1 {
        None => (),
        Some(dir) => match dir {
            'U' => tail.y += 1,
            'D' => tail.y -= 1,
            'L' => tail.x -= 1,
            'R' => tail.x += 1,
            _ => panic!("???? TF??"),
        }
    }
}

fn move_head(head: &mut Posició, dir: &char) {
    match dir {
        // Top left: --
        // Bottom right: ++
        'U' => head.y += 1,
        'D' => head.y -= 1,
        'L' => head.x -= 1,
        'R' => head.x += 1,
        _ => panic!("Wack-ass direction"),
    }
}

fn find_tail_move(head: &Posició, tail: &Posició) -> (char, Option<char>) {
    if same_row_or_col(head, tail) {
       return (find_orthogonal_move(head, tail), None);
    }
    let d = find_diagonal_move(head, tail); 
    return (d.0, Some(d.1));

}

fn find_diagonal_move(head: &Posició, tail: &Posició) -> (char, char) {
    if head.x > tail.x {
        if head.y < tail.y { // head is above-right
            ('D', 'R')
        } else { // head is below-right
            ('U', 'R')
        }
    } else {
        if head.y < tail.y { // head is above-left
            ('D', 'L')
        } else { // head is below-left
            ('U', 'L')
        }
    }
}

fn find_orthogonal_move(head: &Posició, tail: &Posició) -> char {
    if head.x > tail.x {
        return 'R';
    } else if head.x < tail.x {
        return 'L';
    } else { // Same x
        if head.y > tail.y {
            return 'U';
        } else {
            return 'D';
        }
    }
}


fn is_close(head: &Posició, tail: &Posició) -> bool {
    if head.x.abs_diff(tail.x) <= 2 || head.y.abs_diff(tail.y) <= 2 {
        true
    } else {
        false
    }
}

fn same_row_or_col(head: &Posició, tail: &Posició) -> bool {
    if head.x != tail.x && head.y != tail.y {
        false
    } else {
        true
    }
}
fn is_touching(head: &Posició, tail: &Posició) -> bool {
    if head.x.abs_diff(tail.x) <= 1 || head.y.abs_diff(tail.y) <= 1 {
        true
    } else {
        false
    }
}

fn split_entrada<'a>(entrada: &'a str) -> Vec<Vec<&'a str>> {
    let seq_d_strings: Vec<Vec<&str>> = entrada.clone().lines()
        .map(|l: &str| l.split(' ').collect()).collect();

    seq_d_strings

}
fn entrada<'a>() -> String {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    if EXEMPLE {
        // fitxer = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string(); // Part 1 example (ans:
        // 13)
        fitxer = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n".to_string();
    }


    fitxer
}
