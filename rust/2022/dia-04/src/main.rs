use std::fs;

fn main() {
    let entrada = entrada_to_vec();

    println!("Hello, world!");

    let mut cnt: i32 = 0;
    for pair in &entrada {
        let split: Vec<String> = pair.split(',').map(|s| s.to_string()).collect();
        let (primer, segon) = (split[0].clone(), split[1].clone());

        let primer: Vec<i32> = primer.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let primer = [primer[0], primer[1]];
        let segon: Vec<i32> = segon.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let segon = [segon[0], segon[1]];

        let primer_r = primer[0]..=primer[1];
        let segon_r = segon[0]..=segon[1];
        dbg!(&primer_r);
        let mut counted = false;
        for i in primer_r.clone() {
            if !counted && segon_r.contains(&i) {cnt += 1; counted = true;}
        }
        for i in segon_r.clone() {
            if !counted && primer_r.contains(&i) {cnt += 1; counted = true; }
        }
        //
        //if primer[0] <= segon[0] || primer[1] >= segon[1] {
        //    println!("{:?} contains {:?}", primer, segon);
        //    cnt += 1;
        //} else if primer[0] >= segon[0] || primer[1] <= segon[1] {
        //    println!("{:?} contains {:?}", segon, primer);
        //    cnt += 1;
        //}
        //else {
        //    println!("Nope");
        //}
        //if primer[0] <= segon[0] && primer[1] < segon[0] {
        //    println!("{:?} contains {:?}", primer, segon);
        //    cnt += 1;
        //} else if segon[0] <= primer[0] && segon[1] < primer[0] {
        //    println!("{:?} contains {:?}", segon, primer);
        //    cnt += 1;
        //} else {
        //    println!("{:?} DOES NOT CONTAIN {:?}", primer, segon);
        //}
    }

    println!("Resposta és: {cnt}");
}

fn entrada_to_vec() -> Vec<String> {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    let fitxer_e = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let seq_d_strings: Vec<String> = fitxer.split('\n').map(|s| s.to_string()).collect();

    seq_d_strings
}
