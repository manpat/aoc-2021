#![feature(type_ascription, drain_filter)]



fn main() {
	let input = include_str!("day4.txt").replace('\r', "");

	let winning_numbers_str = input.lines().next().unwrap();
	let winning_numbers = winning_numbers_str.split(',')
		.filter_map(|s| s.parse().ok())
		.collect(): Vec<i32>;

	let boards = input.split("\n\n")
		.skip(1)
		.filter_map(parse_board_data)
		.collect(): Vec<BoardData>;

	part_1(&winning_numbers, &boards);
	part_2(&winning_numbers, &boards);
}


fn part_1(winning_numbers: &[i32], boards: &[BoardData]) {
	let mut boards = boards.iter()
		.map(|data| Board::new(data))
		.collect(): Vec<_>;

	for &number in winning_numbers {
		for board in boards.iter_mut() {
			board.mark_value(number);

			if board.win_condition_met() {
				let sum = board.unmarked_values_sum();
				println!("winning board score: {}*{} = {}", sum, number, sum*number);
				return;
			}
		}
	}

	panic!("no boards won");
}


fn part_2(winning_numbers: &[i32], boards: &[BoardData]) {
	let mut boards = boards.iter()
		.map(|data| Board::new(data))
		.collect(): Vec<_>;

	let mut winning_boards_and_numbers = Vec::with_capacity(boards.len());

	for &number in winning_numbers {
		for board in boards.iter_mut() {
			board.mark_value(number);
		}

		let newly_winning_boards = boards.drain_filter(|b| b.win_condition_met())
			.zip(std::iter::repeat(number));

		winning_boards_and_numbers.extend(newly_winning_boards);
	}

	let (board, winning_number) = winning_boards_and_numbers.last().unwrap();
	let sum = board.unmarked_values_sum();
	println!("last win board score: {}*{} = {}", sum, winning_number, sum*winning_number);
}




type BoardData = [i32; 5*5];

fn parse_board_data(s: &str) -> Option<BoardData> {
	use std::str::FromStr;

	let values = s.split_ascii_whitespace().map(i32::from_str).filter_map(Result::ok);
	let mut board = [0; 5*5];

	for (cell, value) in board.iter_mut().zip(values) {
		*cell = value;
	}

	Some(board)
}



struct Board<'d> {
	data: &'d BoardData,
	marks: [bool; 5*5],
}

impl<'d> Board<'d> {
	fn new(data: &'d BoardData) -> Self {
		Board {
			data,
			marks: [false; 5*5]
		}
	}

	fn mark_value(&mut self, value: i32) {
		if let Some((mark, _)) = self.marks.iter_mut()
			.zip(self.data)
			.find(|(_, &cell)| cell == value)
		{
			*mark = true;
		}
	}

	fn unmarked_values_sum(&self) -> i32 {
		self.data.iter().zip(&self.marks)
			.filter(|(_, &mark)| !mark)
			.map(|(&value, _)| value)
			.sum(): i32
	}

	fn win_condition_met(&self) -> bool {
		for row in 0..5 {
			if self.is_row_filled(row) {
				return true;
			}
		}

		for column in 0..5 {
			if self.is_column_filled(column) {
				return true;
			}
		}

		false
	}

	fn is_row_filled(&self, row: usize) -> bool {
		self.marks[row*5 .. (row+1)*5].iter().all(|&m| m)
	}

	fn is_column_filled(&self, column: usize) -> bool {
		self.marks[column..].iter()
			.step_by(5)
			.all(|&m| m)
	}
}