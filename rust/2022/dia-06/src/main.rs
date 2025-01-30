use std::{fs, collections::HashMap};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let window_size = 14; // 4 per a la primera part, 14 per la segona
    let test_entrada = test_entrada();
    let entrada = entrada();
    for pair in test_entrada {
        println!("{} hauria de ser {}, és {}", pair.0, pair.1, trobar_primer_marcador(&pair.0, window_size))
    }

    println!("Resposta: {}", trobar_primer_marcador(&entrada, window_size));
}

fn trobar_primer_marcador(entrada: &str, window_size: usize) -> usize {
    //let mut iter = UnicodeSegmentation::graphemes(entrada, true).map(|s| &s[0] as char).collect::<Vec<char>>().windows(4);
    let binding = entrada.chars().collect::<Vec<char>>();
    let iter = &binding.windows(window_size);
    'main_loop: for (i, seq) in iter.clone().into_iter().enumerate() {
        for c in seq {
            if (seq.to_owned().iter().collect::<String>()).matches(*c).count() != 1 {
                continue 'main_loop;
            }
        }
        return i + window_size;
    }

    unreachable!()

}
fn entrada() -> String {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    fitxer
}

fn test_entrada() -> HashMap<String, usize> {
    HashMap::from([
        (String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7),
        (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5),
        (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 6),
        (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10),
        (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11),
    ])
}
