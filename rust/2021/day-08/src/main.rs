use std::{fs, collections::HashMap};

fn main() {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");
    let entrada = entrada_to_vec(&fitxer);

    let wires_per_digit = HashMap::from([ (0, 6), (1, 2), (2, 5), (3, 5), (4, 4), (5, 5), (6, 6), (7, 3), (8, 7), (9, 6), ]);
    let unique_numbers = [2, 3, 4, 7];
    for parella in entrada {
        let mut correspondencia: HashMap<usize, usize> = HashMap::new(); // 
        let primera_seq  = parella[0].clone();
        for seq in primera_seq {
            if unique_numbers.contains(&seq.len()) {
                correspondencia.insert(k, v)
            }

        }

    }

}

fn fake_entrada_to_vec(s: &String) -> Vec<[Vec<char>; 2]> {
    let lines: Vec<&str> = s.split('\n').collect::<Vec<&str>>();
    let split_lines: Vec<[Vec<char>; 2]> = {
        let mut tmp: Vec<[Vec<char>; 2]> = Vec::new();
        for line in lines {
            let line_contents = line.split('|').collect::<Vec<&str>>();
            let left = line_contents[0].chars().collect::<Vec<char>>();
            let right = line_contents[1].chars().collect::<Vec<char>>();
            let inner_array = [left, right];
            tmp.push(inner_array);
        }
        tmp
    };

    split_lines


}
fn entrada_to_vec(s: &String) -> Vec<[Vec<Vec<char>>; 2]> {
    let lines: Vec<&str> = s.split('\n').collect::<Vec<&str>>();
    let split_lines = {
        let mut tmp: Vec<[Vec<Vec<char>>; 2]> = Vec::new();
        for line in lines {
            let line_contents = line.split('|').collect::<Vec<&str>>();
            let left = line_contents[0].split(' ').collect::<Vec<&str>>();
            let left_seq: Vec<usize> = {
                let mut tmp = Vec::new() ;
                for seq in &left {
                    tmp.push(seq.len());
                }
                tmp
            };
            let left_c: Vec<Vec<char>> = {
                let mut tmp = Vec::new() ;
                for seq in &left.clone() {
                    // seq: &str --> Vec<char>
                    let ve: Vec<char> = (*seq).chars().collect();
                    tmp.push(ve);
                }
                tmp
            };

            let right = line_contents[1].split(' ').collect::<Vec<&str>>();
            let right_seq: Vec<usize> = {
                let mut tmp = Vec::new() ;
                for seq in &right {
                    tmp.push(seq.len());
                }
                tmp
            };

            let right_c: Vec<Vec<char>> = {
                let mut tmp = Vec::new() ;
                for seq in &right {
                    // seq: &str --> Vec<char>
                    let ve: Vec<char> = (*seq).chars().collect();
                    tmp.push(ve);
                }
                tmp
            };

            let inner_array = [left_c, right_c];
            tmp.push(inner_array);
        }
        tmp
    };

    split_lines


}
