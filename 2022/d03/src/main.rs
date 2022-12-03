use itertools::Itertools;
use std::collections::HashMap;

fn main() {
	let uppercase = 'A'..='Z';
	let lowercase = 'a'..='z';
	let mut priorities = HashMap::with_capacity(52);
	for (i, char) in lowercase.enumerate() {
		priorities.insert(char, i + 1);
	}
	for (i, char) in uppercase.enumerate() {
		priorities.insert(char, i + 27);
	}
	let count = include_str!("../input.txt")
		.split('\n')
		.fold(0, |acc, line| {
			let (first, last) = line.split_at(line.len() / 2);
			for i in first.chars() {
				if last.contains(i) {
					return acc + priorities[&i];
				}
			}
			0
		});
	println!("part 1: {}", count);

	let count: usize = include_str!("../input.txt")
		.lines()
		.map(|l| l.as_bytes())
		.collect::<Vec<_>>()
		.iter()
		.tuples()
		.map(|(a, b, c)| same_chars(a, &same_chars(b, c)))
		.map(|c| priorities[&char::from(c[0])])
		.sum();

	println!("part 2: {}", count);
}

fn same_chars(a: &[u8], b: &[u8]) -> Vec<u8> {
	a.iter().copied().filter(|c| b.contains(c)).collect()
}
