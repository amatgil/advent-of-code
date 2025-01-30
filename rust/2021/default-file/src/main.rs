use std::{fs, num::ParseIntError};

fn main() {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");
    let entrada = entrada_to_vec(&fitxer);

    println!("Hello, world!");
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
