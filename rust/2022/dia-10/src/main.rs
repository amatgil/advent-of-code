use std::fs;

const EXEMPLE: bool = false;
const MIDA_HORITZONTAL: usize = 40;
const MIDA_VERTICAL: usize = 6;

fn main() {
    let entrada = entrada_to_vec();

    let instruccions = entrada;

    let mut register = 1;
    let mut cycle = 0;
    let mut cnt = 0;

    println!("Part 2:"); // cnt és la resposta de la Part 1
    for instrucció in instruccions {
        let paraules: Vec<&str> = instrucció.split(' ').collect();

        match paraules[0] {
            "noop" => {
                cycle_increment(&mut register, &mut cycle, &mut cnt);
            },
            "addx" => {
                let delta: i32 = paraules[1].parse().unwrap();
                cycle_increment(&mut register, &mut cycle, &mut cnt);
                cycle_increment(&mut register, &mut cycle, &mut cnt);
                register += delta;
            },
            _ => panic!("Wack instruction: {}", instrucció),
        }
    }

    println!();
    println!("Part 1: {cnt}"); // cnt és la resposta de la Part 1

}

fn draw_pixel(reg: &i32, cycle: &i32) {
    let row = cycle / 40;
    let delta_cycle = *cycle - 40 * row;

    if *reg == delta_cycle || reg - 1 == delta_cycle || reg + 1 == delta_cycle {
        print!("#");
    } else {
        print!(".");
    }
    if (cycle + 1) % 40 == 0 {
        println!();
    }

}
fn cycle_increment(reg: &mut i32, cycle: &mut i32, cnt: &mut i32) {
    draw_pixel(reg, cycle);
    *cycle += 1;

    if *cycle % 40 == 20 {
        *cnt += *cycle * *reg;
    }
}
fn entrada_to_vec() -> Vec<String> {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");

    if EXEMPLE {
        fitxer = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string()
    }

    let seq_d_strings: Vec<String> = fitxer.lines().map(|s| s.to_string()).collect();

    seq_d_strings
}
