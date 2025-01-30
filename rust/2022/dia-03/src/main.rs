use std::fs;

fn main() {
    let entrada = entrada();

    let mut badges: Vec<char> = Vec::new();


    for i in (0..entrada.len()).step_by(3) {
        let elf_1 = &entrada[i];
        let elf_2 = &entrada[i+1];
        let elf_3 = &entrada[i+2];
        let item_en_comú = 'label_to_return_to: {
            for it in 'a' as u8..='z' as u8 {
                let item_actual = it as char;
                if elf_1.contains(item_actual) && elf_2.contains(item_actual) && elf_3.contains(item_actual) {
                    break 'label_to_return_to item_actual;
                }
            }
            for it in 'A' as u8..='Z' as u8 {
                let item_actual = it as char;
                if elf_1.contains(item_actual) && elf_2.contains(item_actual) && elf_3.contains(item_actual) {
                    break 'label_to_return_to item_actual;
                }
            }
            unreachable!()
        };

        badges.push(item_en_comú);
    }

    let resposta = badges.into_iter().map(|b| score_item(b)).sum::<i32>();

    println!("Resposta és: {}", resposta);

    //let mut prioritats = Vec::new();
    //for rucksack in entrada {
    //    let total_length = rucksack.len();
    //    let comp_length = total_length/2;
    //    let comp_1 = &rucksack[0..comp_length];
    //    let comp_2 = &rucksack[comp_length..total_length];
    //    dbg!(comp_1, comp_2);

    //    let item: char = 'label_to_return_to: {
    //        for it in 'a' as u8..='z' as u8 {
    //            let item_actual = it as char;
    //            if comp_1.contains(item_actual) && comp_2.contains(item_actual) {
    //                break 'label_to_return_to item_actual;
    //            }
    //        }
    //        for it in 'A' as u8..='Z' as u8 {
    //            let item_actual = it as char;
    //            if comp_1.contains(item_actual) && comp_2.contains(item_actual) {
    //                break 'label_to_return_to item_actual;
    //            }
    //        }

    //        unreachable!()
    //    };

    //    let val = score_item(item);
    //    prioritats.push(val);

    //}

    //println!("La resposta és: {}", prioritats.into_iter().sum::<i32>());

    // Find both sides
    // Compare
    // Find the item in both compartments
    // Find the priority of the item in rucksack
}

fn entrada_to_vec<'a>(s: &'a String) -> Vec<String> {
    let slice: &'a str = &s[..];
    let seq_d_strings: Vec<String> = slice.split('\n').map(|refe| refe.to_string()).collect();
    seq_d_strings
}

fn score_item(item: char) -> i32 {
    let i_code = item as u8;
    if item.is_lowercase() {
        let a_code = 'a' as u8;
        let resposta = i_code - a_code + 1;
        resposta as i32

    } else {
        let a_code = 'A' as u8; 
        let resposta = i_code - a_code + 1;
        26 + resposta as i32
    }
}

fn entrada<'a>() -> Vec<String> {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

//    let fitxer = "vJrwpWtwJgWrhcsFMMfFFhFp
//jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//PmmdzqPrVvPwwTWBwg
//wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//ttgJtRGJQctTZtZT
//CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    let entrada = entrada_to_vec(&fitxer);
    entrada
}
