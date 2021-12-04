#![feature(type_ascription, slice_group_by)]


fn main() {
	let input = include_str!("day1.txt").lines()
		.filter_map(|l| l.parse().ok())
		.collect(): Vec<i32>;

	// part 1
	let num_increases = input.group_by(|&a, &b| a < b)
		.map(|slice| slice.len()-1).sum(): usize;

	dbg!(num_increases);

	// part 2
	let sums = input.windows(3)
		.map(|slice| slice.iter().cloned().sum(): i32)
		.collect(): Vec<_>;


	let num_increases = sums.group_by(|&a, &b| a < b)
		.map(|slice| slice.len()-1).sum(): usize;

	dbg!(num_increases);

}