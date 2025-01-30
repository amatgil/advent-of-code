use std::{fs, num::ParseIntError};

fn main() {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("File probably doesn't exist yet");
    fitxer.pop().expect("Entrada incorrecta?");
    let entrada = entrada_to_vec(&fitxer).unwrap();

    println!("Hello, world!");

    //let average = entrada.clone().into_iter().sum::<u128>() as f64 / entrada.clone().len() as f64;
    let mut min_fuel = 100000000;
    let mut sortida = 0;

    for p in 0..entrada.len() {
        sortida = 0;
        for v in entrada.clone() {
            //let delta = crab_distance((p as u128).abs_diff(v)); // Part 1
            let delta = crab_distance((p as u128).abs_diff(v)); // Part 2
            sortida += delta;
        }
        min_fuel = std::cmp::min(min_fuel, sortida);
        println!("p: {}, fuel: {}; min: {}", p, sortida, min_fuel);
    }

    println!("Sortida: {sortida} amb fuel {min_fuel}");
}

fn crab_distance(s: u128) -> u128 {
    s * (s+1) / 2


}
fn entrada_to_vec(s: &String) -> Result<Vec<u128>, ParseIntError> {
    let seq_d_strings: Vec<&str> = s.split(',').collect();
    let mut sortida = Vec::new();
    for string in seq_d_strings {
        let n = string.parse::<u128>()?;
        sortida.push(n);
    }


    Ok(sortida)
}
