#![feature(type_ascription)]



fn main() {
	let input = include_str!("day3.txt").lines()
		.filter_map(|l| u32::from_str_radix(l, 2).ok())
		.collect(): Vec<u32>;


	// part 1
	let differences = input.iter()
		.fold([0; 12], |mut acc, &value| {
			for (bit_acc, bit_pos) in acc.iter_mut().zip(0..12) {
				match is_bit_set(value, bit_pos) {
					true => *bit_acc += 1,
					false => *bit_acc -= 1,
				};
			}

			acc
		});

	let gamma = differences.iter().enumerate()
		.fold(0, |acc, (pos, &diff)| {
			let bit_value = match diff > 0 {
				false => 0,
				true => 1,
			};

			acc | bit_value << pos
		});

	let epsilon = (!gamma) & ((1<<12) - 1);

	println!("{:b} * {:b} = {2:b} ({2})", gamma, epsilon, gamma * epsilon);

	// part 2
	let oxy_rating = calculate_rating(&input, Rating::Oxygen);
	let co2_rating = calculate_rating(&input, Rating::CO2);

	dbg!(oxy_rating * co2_rating);
}


enum Rating {
	Oxygen,
	CO2,
}

fn calculate_rating(values: &[u32], rating: Rating) -> u32 {
	let mut values = values.to_owned();

	for bit_position in (0..12).rev() {
		let most_common = most_common_bit_value(&values, bit_position);
		let bit_criteria = match rating {
			Rating::Oxygen => most_common,
			Rating::CO2 => !most_common,
		};

		values.retain(|&v| is_bit_set(v, bit_position) == bit_criteria);
		if values.len() == 1 {
			return values[0];
		}
	}

	panic!("Couldn't caluclate rating")
}



fn most_common_bit_value(values: &[u32], bit_pos: u32) -> bool {
	let accum = values.iter().fold(0, |acc, &value| {
		match is_bit_set(value, bit_pos) {
			false => acc - 1,
			true => acc + 1,
		}
	});

	accum >= 0
}


fn is_bit_set(value: u32, position: u32) -> bool {
	(value & (1<<position)) != 0
}