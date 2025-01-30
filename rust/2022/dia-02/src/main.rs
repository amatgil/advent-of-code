fn main() {
    println!("Hello, world!");
    let input = {
        let mut i =input();
        i.pop();
        i
    };
    dbg!(&input);
    let mut moves: Vec<&str> = input.split('\n').collect();

    let points_for_selected = {
        let mut rocks = 0;
        let mut papers = 0;
        let mut scissors = 0;
        for moviment in moves {
            let used = get_used(moviment);
            match used {
                'A' => rocks += 1,
                'B' => papers += 1,
                'C' => scissors += 1,
                _ => unreachable!(),
            }
        }
        dbg!(rocks, papers, scissors);
        rocks + papers * 2 + scissors * 3
    };

    let points_for_winning = {
        let guanya = input.matches('Z').count();
        let empat = input.matches('Y').count();
        let perdua = input.matches('X').count();

        guanya * 6 + empat * 3 + perdua * 0
    };


    //let points_for_winning = {
    //    let mut count = 0;
    //    for mov in moves {
    //        count += match battle(mov) {
    //            Result::Win => 6,
    //            Result::Draw => 3,
    //            Result::Loss => 0,
    //        };
    //    }
    //    count
    //};


    dbg!(points_for_selected);
    dbg!(points_for_winning);
    //dbg!(moves);
    println!("La resposta és: {}", points_for_selected + points_for_winning);
}
#[derive(Debug)]
enum Result {
    Win,
    Loss,
    Draw,
}

fn get_used(moviment: &str) -> char {
    let frase: Vec<char> = moviment.chars().collect();
    let primer = frase[0];
    let segon = frase[2];
    if primer == 'A' { // Rock
        if segon == 'X' { return 'C' } // Loss
        if segon == 'Y' { return 'A' } // Draw
        if segon == 'Z' { return 'B' } // Win
    } else if primer == 'B' { // Paper
        if segon == 'X' { return 'A' } // Loss
        if segon == 'Y' { return 'B' } // Draw
        if segon == 'Z' { return 'C' } // Win
    } else if primer == 'C' { // Tisores 
        if segon == 'X' { return 'B'} // Loss
        if segon == 'Y' { return 'C'} // Draw
        if segon == 'Z' { return 'A'} // Win
    }

    panic!()
}

//fn process_moves(moviments_originals: Vec<&str>) -> Vec<&str> {
//    let mut sortida = Vec::new();
//    for mov in moviments_originals {
//        let frase: Vec<char> = mov.chars().collect();
//        let primer = frase[0];
//        let segon = frase[2];
//        if primer == 'A' { // Rock
//            if segon == 'X' { sortida.append(&mut Vec::from("A C"))} // Loss
//            if segon == 'Y' { sortida.append(&mut Vec::from("A A"))} // Draw
//            if segon == 'Z' { sortida.append(&mut Vec::from("A B"))} // Win
//        } else if primer == 'B' { // Paper
//            if segon == 'X' { sortida.append(&mut Vec::from("B A"))} // Loss
//            if segon == 'Y' { sortida.append(&mut Vec::from("B B"))} // Draw
//            if segon == 'Z' { sortida.append(&mut Vec::from("B C"))} // Win
//        } else if primer == 'C' { // Tisores 
//            if segon == 'X' { sortida.append(&mut Vec::from("C B"))} // Loss
//            if segon == 'Y' { sortida.append(&mut Vec::from("C C"))} // Draw
//            if segon == 'Z' { sortida.append(&mut Vec::from("C A"))} // Win
//    }
//
//    return sortida;
//
//}
fn battle(moviment: &str) -> Result {
    let mov: Vec<char> = moviment.chars().collect();
    if mov[0] == 'A' {
        if mov[2] == 'X' { return Result::Draw; }
        if mov[2] == 'Y' { return Result::Win; }
        if mov[2] == 'Z' { return Result::Loss; }
    } else if mov[0] == 'B' {
        if mov[2] == 'X' { return Result::Loss; }
        if mov[2] == 'Y' { return Result::Draw; }
        if mov[2] == 'Z' { return Result::Win; }
    } else if mov[0] == 'C' {
        if mov[2] == 'X' { return Result::Win; }
        if mov[2] == 'Y' { return Result::Loss; }
        if mov[2] == 'Z' { return Result::Draw; }
    }
    
    panic!()
}

fn input() -> String {
    let example = "A Y
B X
C Z
";
    include_str!("../input.data").to_string()
    //example.to_string()
}
