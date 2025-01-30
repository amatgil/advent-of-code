use std::{fs, num::ParseIntError, cmp::Ordering};

fn main() {
    let mut fitxer =
        fs::read_to_string("./input.data").expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");
    let entrada = entrada_to_vec(&fitxer);

    let mut list_of_heights: Vec<usize> = Vec::new();
    for y in 0..entrada.len() {
        'inner: for x in 0..entrada[0].len() {
            let shifted_delta_arr = [0, 1, 2];
            for d in shifted_delta_arr {
                let compared_x = entrada[y].get(x+d-1).cmp(&Some(&entrada[y][x]));
                if let Ordering::Less = compared_x {
                    continue 'inner;
                }
            }
            list_of_heights.push(entrada[y][x]);
            }
        }
    }
    dbg!(&list_of_heights);
    let risk_levels = list_of_heights.into_iter().map(|h| h + 1);
    println!("Result: {}", risk_levels.into_iter().sum::<usize>());
}

fn entrada_to_vec(s: &String) -> Vec<Vec<usize>> {
    let seq_d_strings: Vec<&str> = s.split(',').collect();
    let mut sortida: Vec<Vec<usize>> = Vec::new();
    for string in seq_d_strings {
        let nums: Vec<usize> = string
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        sortida.push(nums);
    }

    sortida
}
