use itertools::Itertools;

fn main() {
	let part_1 = include_str!("../input.txt")
		.chars()
		.tuple_windows::<(_, _, _, _)>()
		.enumerate()
		.find(|(_key, (a, b, c, d))| a != b && a != c && a != d && b != c && b != d && c != d)
		.or(Some((0, ('X', 'X', 'X', 'X'))));

	// + 3 to offset the window + 1 to offset enumerate starting with 0
	println!("Part 1: {:?}", part_1.unwrap().0 + 4);
	let part_2 = include_str!("../input.txt")
		.as_bytes()
		.windows(14)
		.enumerate()
		.find_map(|(i, window)| {
			if window.iter().all_unique() {
				Some(i + 14)
			} else {
				None
			}
		});
	println!("Part 2: {}", part_2.unwrap());
}
