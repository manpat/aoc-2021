#![feature(type_ascription, slice_group_by)]

fn main() {
	let input = include_str!("day2.txt").lines()
		.filter_map(parse_command)
		.collect(): Vec<Command>;

	// part 1
	let (position, depth) = input.iter()
		.fold((0, 0), |(curr_pos, curr_depth), command| {
			let (pos_diff, depth_diff) = match *command {
				Command::Forward(dist) => (dist, 0),
				Command::Down(dist) => (0, dist),
				Command::Up(dist) => (0, -dist),
			};

			(curr_pos + pos_diff, curr_depth + depth_diff)
		});

	dbg!(position * depth);

	// part 2

	let (position, depth, _) = input.iter()
		.fold((0, 0, 0), |(curr_pos, curr_depth, curr_aim), command| {
			let (pos_diff, depth_diff, aim_diff) = match *command {
				Command::Forward(dist) => (dist, dist*curr_aim, 0),
				Command::Down(amt) => (0, 0, amt),
				Command::Up(amt) => (0, 0, -amt),
			};

			(curr_pos + pos_diff, curr_depth + depth_diff, curr_aim + aim_diff)
		});

	dbg!(position * depth);
}


#[derive(Debug, Copy, Clone)]
enum Command {
	Forward(i32),
	Down(i32),
	Up(i32),
}

fn parse_command(s: &str) -> Option<Command> {
	let mut it = s.split_ascii_whitespace();
	let name = it.next()?;
	let arg = it.next()?.parse().ok()?: i32;

	match name {
		"forward" => Some(Command::Forward(arg)),
		"down" => Some(Command::Down(arg)),
		"up" => Some(Command::Up(arg)),
		_ => None,
	}
}
