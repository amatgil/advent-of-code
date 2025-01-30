use std::{fs, num::ParseIntError};

fn main() {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    let mut peixets: [u128; 9] = vec_to_peixets(entrada_to_vec(&fitxer).expect("Entrada incorrecta"));
    let mut nous_peixets = [0; 9];

    for d in 0..256 {
        for n in 0..8 {
            nous_peixets[n] = peixets[n+1];
        }
        nous_peixets[8] = peixets[0];
        nous_peixets[6] += peixets[0];
        peixets = nous_peixets;
    }

    println!("Resultat final: {}", peixets.into_iter().sum::<u128>());
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

fn vec_to_peixets(v: Vec<u128>) -> [u128; 9] {
    let mut sortida = [0; 9];

    for val in v {
        sortida[val as usize] += 1;
    }

    sortida
}
