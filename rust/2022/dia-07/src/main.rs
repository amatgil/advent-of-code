use std::fs;

use nom::{IResult, branch::alt, character::complete::{i32, alpha1, digit1}, bytes::complete::tag, multi::separated_list1}; 

enum Tree<'a> {
    Dir {
        name: &'a str,
        contents: Option<Vec<Tree<'a>>>,
    },
    File(usize)
}

fn parse_input(input: &str) -> IResult<&str, Tree> {
    let (input, cmd_or_output) = alt((
            alpha1,
            digit1,
            tag("$"),
        ))(input)?;

    let tree = Tree::Dir { name: "/", contents: None };
    let subtree_strings = input.split("$ cd");
    match cmd_or_output {
        "$" => {
            let (input, cmd) = alt((
                    tag("cd"),
                    tag("ls"),
                ))(input)?;
            match cmd {
                "cd" => {
                    let (input, dir_name) = i32(input)?;
                    tree.
                },
                "ls" => {},
                x => panic!("Unrecognized command: {x}"),
            }
        },
        "dir" => {
            println!("It's a dir");
        },
        x if x.parse::<i32>().is_ok() => {
            println!("It's a file");
        },
        x => panic!("Unrecognized prefix: {x}"),
    }

    Ok((input, tree))
}

fn main() {
    let input = entrada();
    let (input, parsed) = parse_input(&input).unwrap();
    println!("{:?}\n\n{:?}", input, parsed);


    println!("{:?}", &input);
}

fn entrada() -> String {
    let mut fitxer = fs::read_to_string("./input.data")
        .expect("Should have been able to read the file");
    fitxer.pop().expect("Entrada incorrecta?");
    let fitxer = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
 
    fitxer.to_string()
}
