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

	let mut last_winning_board_and_number = None;

	for &number in winning_numbers {
		for board in boards.iter_mut() {
			board.mark_value(number);
		}

		if let Some(winning_board) = boards.drain_filter(|b| b.win_condition_met()).last() {
			last_winning_board_and_number = Some((winning_board, number));
		}
	}

	let (board, winning_number) = last_winning_board_and_number.unwrap();
	let sum = board.unmarked_values_sum();
	println!("last win board score: {}*{} = {}", sum, winning_number, sum*winning_number);
}




type BoardData = [i32; 5*5];

fn parse_board_data(s: &str) -> Option<BoardData> {
	let values = s.split_ascii_whitespace().filter_map(|s| s.parse().ok());
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
		(0..5).any(|idx| self.is_row_filled(idx) || self.is_column_filled(idx))
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