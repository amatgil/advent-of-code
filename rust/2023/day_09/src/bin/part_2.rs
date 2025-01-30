const USING_TEST_INPUT: bool = false;

use std::{error::Error, collections::VecDeque};

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
	let input = {
		if USING_TEST_INPUT {
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".into()
		} else {
			gai::load_input(1, 9)?
		}
	};

	let input: Vec<Vec<i32>> = input.lines()
		.map(|s| s.split(' ').collect::<Vec<&str>>())
		.map(|v| v.iter().map(|s| s.parse::<i32>().unwrap()).collect() )
		.collect();

	let mut seqs_to_extend: Vec<VecDeque<VecDeque<i32>>> = vec![];
	for seq in &input {
		let mut seq_list: VecDeque<VecDeque<i32>> = VecDeque::from([seq.clone().into()]);
		let mut last_seq: VecDeque<i32> = seq.clone().into(); 
		while !is_zeros(&last_seq) {
			let new_seq: VecDeque<i32> = get_new_seq(&last_seq);
			seq_list.push_front(new_seq.clone());
			last_seq = new_seq.clone();
		}
		seqs_to_extend.push(seq_list);
	}
	let mut extended_seqs = vec![];
	for seq in seqs_to_extend {
		let mut seq: VecDeque<VecDeque<i32>> = seq.into_iter().collect(); // Zeros at the front
		seq[0].push_front(0);
		for i in 1..seq.len() {
			let n = seq[i].iter().next().unwrap() - seq[i - 1].iter().next().unwrap();
			seq[i].push_front(n);
		}
		extended_seqs.push(seq);
	}

	let values: Vec<_> = extended_seqs.into_iter()
        .map(|s| *s.into_iter().last().unwrap().iter().next().unwrap())
        .collect();

	println!("The answer is: {}", values.into_iter().sum::<i32>());
	Ok(())
}

fn is_zeros(v: &VecDeque<i32>) -> bool { !v.iter().any(|&x| x != 0) }

fn get_new_seq(seq: &VecDeque<i32>) -> VecDeque<i32>{ // There's no windows on VecDeque!!
    let mut out = VecDeque::with_capacity(seq.len() - 1);
    for i in 0..seq.len() - 1 { out.push_back(seq[i+1] - seq[i]); }
    out 
}
