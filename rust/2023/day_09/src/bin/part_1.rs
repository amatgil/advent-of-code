use std::error::Error;

const USING_TEST_INPUT: bool = false;

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

	let input: Vec<Vec<isize>> = input.lines()
		.map(|s| s.split(' ').collect::<Vec<&str>>())
		.map(|v| v.iter().map(|s| s.parse::<isize>().unwrap()).collect() )
		.collect();

	let mut seqs_to_extend = vec![];
	for seq in &input {
		let mut seq_list = vec![seq.clone()];
		let mut last = seq.clone(); 
		while !is_zeros(&last) {
			let new_seq = last.windows(2).map(|v| v[1] - v[0]).collect::<Vec<isize>>();
			seq_list.push(new_seq.clone());
			last = new_seq.clone();
		}
		seqs_to_extend.push(seq_list);
	}
	let mut extended_seqs = vec![];
	for seq in seqs_to_extend {
		let mut seq: Vec<Vec<isize>> = seq.into_iter().rev().collect(); // Zeros at the front
		seq[0].push(0);
		for i in 1..seq.len() {
			let n = *seq[i - 1].last().unwrap() + seq[i].last().unwrap();
			seq[i].push(n);
		}
		extended_seqs.push(seq);
	}

	let values: Vec<_> = extended_seqs.into_iter().map(|s| *s.last().unwrap().last().unwrap()).collect();

	println!("The answer is: {}", values.into_iter().sum::<isize>());

	Ok(())
}

fn is_zeros(v: &[isize]) -> bool { !v.iter().any(|&x| x != 0) }
