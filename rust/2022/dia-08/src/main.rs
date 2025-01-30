use std::fs;

const EXEMPLE: bool = true;

#[derive(Debug, Clone, Copy)]
struct Punt {
    x: usize,
    y: usize,
}

impl Punt {
    fn new(x: usize, y: usize) -> Self {
        Punt { x, y }
    }
}

fn main() {
    let entrada = entrada_to_vec();

    let len = entrada.len();
    let mut cnt = 0;
    for x in 0..len {
        for y in 0..len {
            if visible(entrada.clone(), Punt::new(x, y)) {
                println! {"({x}, {y}) és visible (valor: {})", entrada[x][y]};
                cnt += 1;
            }
        }
    }

    println!("Resposta: {cnt}");
}

fn visible(taulell: Vec<Vec<usize>>, pos: Punt) -> bool {
    let len = taulell.len()-1;
    if pos.x == 0 || pos.x == len || pos.y == 0 || pos.y == len {
        false // SHOULD BE TRUE BY THE END; BUT I'M DEBUGGIN
    } else {
        let mut visibility = [true; 4]; // Top, btm, left, right
        for p in 0..pos.x {
            if taulell[p][pos.y] >= taulell[pos.x][pos.y] {
                visibility[2] = false;
            }
        }
        for p in pos.x..len {
            if taulell[p][pos.y] >= taulell[pos.x][pos.y] {
                visibility[3] = false;
            }
        }
        for p in 0..pos.y {
            if taulell[pos.x][p] >= taulell[pos.x][pos.y] {
                visibility[0] = false;
            }
        }
        for p in pos.y..len {
            if taulell[pos.x][p] >= taulell[pos.x][pos.y] {
                visibility[1] = false;
            }
        }
        dbg!(visibility, pos);
        return visibility[0] || visibility[1] || visibility[2] || visibility[3];
    }
}

fn entrada_to_vec() -> Vec<Vec<usize>> {
    let mut fitxer =
        fs::read_to_string("./input.data").expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    if EXEMPLE {
        fitxer = "30373\n25512\n65332\n33549\n35390".to_owned();
    }

    let seq_d_strings: Vec<String> = fitxer.split('\n').map(|s| s.to_string()).collect();

    let r = {
        let mut tmp = Vec::new();
        for s in &seq_d_strings {
            let t: Vec<usize> = s
                .chars() // ['3', '0', '3', '7', '3']
                .map(|c| (c as usize - '0' as usize))
                .collect();
            tmp.push(t);
        }
        tmp
    };
    println!("{:?}", &r);

    r
}
