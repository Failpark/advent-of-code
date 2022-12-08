use std::str::Lines;

fn main() {
	let mut lines = include_str!("../input.txt").lines();
	println!("Part 1: {:?}", sum_subdirs(&mut lines.clone()).into_iter().filter(|&dir_size| dir_size <= 100_000).sum::<usize>());
	let mut dir_sizes = sum_subdirs(&mut lines);
	let space_needed = 30_000_000 - (70_000_000 - dir_sizes.last().unwrap());
	dir_sizes.sort();
	println!("Part 2: {:?}", dir_sizes.into_iter().find(|&dir_size| dir_size >= space_needed).unwrap());
}

fn sum_subdirs(lines: &mut Lines) -> Vec<usize> {
	let mut acc = 0;
	let mut subdirs = vec![];
	while let Some(line) = lines.next() {
		match line.split_whitespace().collect::<Vec<_>>()[..] {
				["$", "cd", ".."] => break,
				["$", "cd", _subdir] => {
					subdirs.extend(sum_subdirs(lines));
					acc += subdirs.last().unwrap();
				},
				["dir", _name] => (),
				["$", _cmd] => (),
				[sum_bytes, _] => {
					let parsed = sum_bytes.parse::<usize>();
					if let Ok(i) = parsed {
						acc += i;
					} else {
						println!("{}", sum_bytes);
					}
				},
				_ => ()
		}
	}

	subdirs.push(acc);
	subdirs
}