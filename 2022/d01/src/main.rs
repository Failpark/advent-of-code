fn main() {
	let (mut heap, _) = format!("{}\n", include_str!("../input.txt"))
		.split('\n')
		.fold(
			(std::collections::BinaryHeap::new(), 0),
			|(mut heap, curr), line| {
				if line.is_empty() {
					heap.push(curr);
					return (heap, 0);
				}

				(heap, curr + line.parse::<i32>().unwrap())
			},
		);

	let part1 = heap.pop().unwrap();
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part1 + heap.iter().take(2).sum::<i32>());
}
