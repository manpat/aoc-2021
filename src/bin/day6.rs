#![feature(type_ascription)]


fn main() {
	let initial_state = include_str!("day6.txt").split(',')
		.filter_map(|s| s.trim().parse().ok())
		.map(LanternFish)
		.collect(): Vec<_>;

	part_1(&initial_state);
	part_2(&initial_state);
}


#[derive(Debug, Copy, Clone)]
struct LanternFish(u8);


fn simulate_n_days(initial_state: &[LanternFish], days: usize) -> u64 {
	// How many fish to spawn each day
	let mut mature_fish_buckets = [0u64; 7];

	// How many fish to spawn each day + become mature
	let mut immature_fish_buckets = [0u64; 9];

	for &LanternFish(days_till_spawn) in initial_state {
		assert!(days_till_spawn < 7, "All fish in initial state must be mature");
		mature_fish_buckets[days_till_spawn as usize] += 1;
	}

	for day in 0..days {
		let mature_fish_bucket = &mut mature_fish_buckets[day % mature_fish_buckets.len()];
		let immature_fish_bucket = &mut immature_fish_buckets[day % immature_fish_buckets.len()];

		// Mature fish spawn new immature fish - immature fish spawn new fish and become mature.
		// Because maturation involves spawning a new fish, logically no fish are ever removed from the
		// immature fish pool, instead new already mature fish are spawned.
		let num_new_adult_fish = *immature_fish_bucket;
		let num_new_immature_fish = *mature_fish_bucket;
		*immature_fish_bucket += num_new_immature_fish;
		*mature_fish_bucket += num_new_adult_fish;
	}

	let num_mature_fish: u64 = mature_fish_buckets.into_iter().sum();
	let num_immature_fish: u64 = immature_fish_buckets.into_iter().sum();
	num_mature_fish + num_immature_fish
}



fn part_1(initial_state: &[LanternFish]) {
	let num_fish = simulate_n_days(initial_state, 80);
	println!("fish after 80 days: {}", num_fish);
}

fn part_2(initial_state: &[LanternFish]) {
	let num_fish = simulate_n_days(initial_state, 256);
	println!("fish after 256 days: {}", num_fish);
}