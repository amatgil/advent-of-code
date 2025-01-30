const EXEMPLE: bool = true;
const DIA: usize = 0;

fn main() {
    let entrada = entrada();

    dbg!(entrada);
}

fn check_list(line: &str) -> bool {
    print!("{}", hi);
}

fn entrada() -> Vec<String> {
    let url = format!("https://adventofcode.com/2022/day/{}/input", DIA);
    let mut text = get_aoc_input::get_input(&url).unwrap();

    if EXEMPLE {
        text = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();
    }

    let seq_d_strings: Vec<String> = text.lines().map(|s| s.to_string()).collect();

    seq_d_strings
}
