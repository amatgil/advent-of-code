use std::fs;

fn main() {
    let entrada = entrada_to_vec();
    let (stacks, instruccions) = entrada.split_at(10);
    dbg!(stacks, instruccions);
    let mut stacks = {
        [
            Vec::from(['S', 'T', 'H', 'F', 'W', 'R']),
            Vec::from(['S', 'G', 'D', 'Q', 'W']),
            Vec::from(['B', 'T', 'W']),
            Vec::from(['D', 'R', 'W', 'T', 'N', 'Q', 'Z', 'J']),
            Vec::from(['F', 'B', 'H', 'G', 'L', 'V', 'T', 'Z']),
            Vec::from(['L', 'P', 'T', 'C', 'V', 'B', 'S', 'G']),
            Vec::from(['Z', 'B', 'R', 'T', 'W', 'G', 'P']),
            Vec::from(['N', 'G', 'M', 'T', 'C', 'J', 'R']),
            Vec::from(['L', 'G', 'B', 'W']),
        ]
    };

    //let (stacks, instruccions) = entrada.split_at(5);
    //let mut stacks = {
    //    [
    //        Vec::from(['Z', 'N']),
    //        Vec::from(['M', 'C', 'D']),
    //        Vec::from(['P']),
    //    ]
    //};

    for instrucció in instruccions {
        let [times, orig, dest] = str_to_inst(instrucció);
        let pila_orig = &mut stacks[orig-1];
        dbg!(times, orig, dest, &pila_orig);
        let a_moure = &(pila_orig.clone())[pila_orig.len() - times..pila_orig.len()];
        for _ in 0..times { pila_orig.pop().unwrap(); }
        for c in a_moure { stacks[dest - 1].push(*c) ; }
        dbg!(a_moure);
    }

    dbg!(&stacks);
    let sortida = {
        let mut tmp = String::new();
        for pila in stacks {
            tmp.push(pila[pila.len() - 1])
        }
        tmp
    };
    println!("Resposta és: {}", sortida);
}

fn str_to_inst(instrucció: &str) -> [usize; 3] {
    let vec: Vec<&str> = instrucció.split(' ').collect();
    let times = vec[1].parse::<usize>().unwrap();
    let origin = vec[3].parse::<usize>().unwrap();
    let dest = vec[5].parse::<usize>().unwrap();

    [times, origin, dest]
}
fn entrada_to_vec() -> Vec<String> {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    let fitxer_e = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let seq_d_strings: Vec<String> = fitxer.split('\n').map(|s| s.to_string()).collect();

    seq_d_strings
}
