use std::ops::RangeInclusive;

fn main() {
	let result = include_str!("../input.txt")
		.split('\n')
		.fold((0, 0), |acc, line| {
			let (first, last) = line.split_once(',').unwrap();
			let (first_a, first_b) = first.split_once('-').unwrap();
			let first_range = RangeInclusive::new(
				first_a.parse::<usize>().unwrap(),
				first_b.parse::<usize>().unwrap(),
			);
			let (last_a, last_b) = last.split_once('-').unwrap();
			let last_range = RangeInclusive::new(
				last_a.parse::<usize>().unwrap(),
				last_b.parse::<usize>().unwrap(),
			);

			if first_range.contains(last_range.start())
				|| first_range.contains(last_range.end())
				|| last_range.contains(first_range.start())
				|| last_range.contains(first_range.end())
			{
				if (first_range.contains(last_range.start())
					&& first_range.contains(last_range.end()))
					|| (last_range.contains(first_range.start())
						&& last_range.contains(first_range.end()))
				{
					(acc.0 + 1, acc.1 + 1)
				} else {
					(acc.0, acc.1 + 1)
				}
			} else {
				acc
			}
		});
	println!("Part 1: {}", result.0);
	println!("Part 2: {}", result.1);
}

// get the longer one, into range, contains start of short & contains end of short
