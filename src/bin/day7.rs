#![feature(type_ascription)]

use aoc_2021::Timer;

fn main() {
	let initial_positions = include_str!("day7.txt").split(',')
		.filter_map(|s| s.trim().parse().ok())
		.collect(): Vec<i32>;

	part_1(&initial_positions);
	part_2(&initial_positions);
}


fn calculate_cost(positions: &[i32], target: i32, cost_fn: impl Fn(i32,i32)->i32) -> i32 {
	positions.into_iter()
		.map(move |&position| cost_fn(position, target))
		.sum()
}


fn minimum_cost<F>(positions: &[i32], cost_fn: F) -> i32
	where F: Fn(i32,i32)->i32 + Copy
{
	let (min, max) = positions.into_iter()
		.fold((i32::MAX, i32::MIN), |(lb, ub), &position| {
			(lb.min(position), ub.max(position))
		});

	(min..max).map(move |target| calculate_cost(positions, target, cost_fn))
		.min()
		.unwrap()
}



fn part_1_cost(position: i32, target: i32) -> i32 {
	(target - position).abs()
}


fn part_2_cost(position: i32, target: i32) -> i32 {
	// Triangle numbers :^)
	let distance = (target - position).abs();
	distance * (distance+1) / 2
}



fn part_1(positions: &[i32]) {
	let _timer = Timer::new();
	println!("minimum cost: {}", minimum_cost(positions, part_1_cost));
}

fn part_2(positions: &[i32]) {
	let _timer = Timer::new();
	println!("minimum cost with better cost: {}", minimum_cost(positions, part_2_cost));
}



#[test]
fn test_part_1() {
	let cost = minimum_cost(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], part_1_cost);
	assert_eq!(cost, 37);
}

#[test]
fn test_part_2() {
	let cost = minimum_cost(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], part_2_cost);
	assert_eq!(cost, 168);
}