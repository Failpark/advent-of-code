fn main() {
	let mut stack: Vec<Vec<char>> = vec![
		vec!['B', 'Q', 'C'],
		vec!['R', 'Q', 'W', 'Z'],
		vec!['B', 'M', 'R', 'L', 'V'],
		vec!['C', 'Z', 'H', 'V', 'T', 'W'],
		vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
		vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
		vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
		vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'],
		vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
	];
	// [Howmany, Start, End]
	let instructions = include_str!("../input.txt")
		.lines()
		.map(|line| 
			line.split(' ')
			.enumerate()
			.filter(|(idx, _str)| (idx % 2) == 1)
			.map(|(_idx, str)| str.parse().unwrap())
			.collect::<Vec<usize>>()
		).collect::<Vec<Vec<usize>>>();

	// Part 1
	let mut stack_1 = stack.clone();
	for i in &instructions{
		for _j in 0..i[0] {
			let char = stack_1[i[1] - 1].pop().unwrap();
			stack_1[i[2] - 1].push(char);
		}
	}
	print!("Part 1: ");
	for i in stack_1.iter_mut() {
		print!("{}", i.pop().unwrap_or(' '));
	}

	println!();
	// Part 2:
	for i in instructions {
		let len = stack[i[1] - 1].len();
		let mut chars = stack[i[1] - 1].drain(
			(len - i[0])..
		).collect::<Vec<_>>();
		stack[i[2] - 1].append(&mut chars);
	}

	print!("Part 2: ");
	for i in stack.iter_mut() {
		print!("{}", i.pop().unwrap_or(' '));
	}
	println!();
}
