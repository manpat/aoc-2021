#![feature(type_ascription)]

use std::iter;
use std::collections::HashSet;

fn main() {
	let grid = parse_grid(include_str!("day9.txt")).unwrap();
	dbg!(minimum_risk_factors_sum(&grid));
	dbg!(largest_basin_sizes_product(&grid));
}


#[derive(Debug)]
struct Grid {
	data: Vec<u8>,
	width: usize,
	height: usize,
}

impl Grid {
	fn get(&self, x: i32, y: i32) -> Option<u8> {
		let x: usize = x.try_into().ok()?;
		let y: usize = y.try_into().ok()?;

		if x >= self.width || y >= self.height {
			return None
		}

		self.data.get(x + y * self.width).copied()
	}


	fn iter_cell_and_position(&self) -> impl Iterator<Item=(u8, i32, i32)> + '_ {
		let width = self.width as i32;
		let height = self.height as i32;

		let positions = (0..height).flat_map(move |y| (0..width).zip(iter::repeat(y)));
		positions.map(|(x, y)| (self.get(x, y).unwrap(), x, y))
	}

	fn iter_neighbors(&self, x: i32, y: i32) -> impl Iterator<Item=(u8, i32, i32)> + '_ {
		let offsets = [
			(-1, 0),
			( 1, 0),
			(0, -1),
			(0,  1),
		];

		offsets.into_iter()
			.filter_map(move |(ox, oy)| {
				self.get(x + ox, y + oy)
					.map(|v| (v, x + ox, y + oy))
			})
	}

	fn iter_minima(&self) -> impl Iterator<Item=(u8, i32, i32)> + '_ {
		self.iter_cell_and_position()
			.filter(|&(cell, x, y)| {
				self.iter_neighbors(x, y).all(|(neighbor, ..)| cell < neighbor)
			})
	}
}


fn parse_grid(s: &str) -> Option<Grid> {
	let width = s.lines().next()?.len();
	let height = s.lines().filter(|line| !line.is_empty()).count();

	let data = s.lines()
		.flat_map(|line| line.chars())
		.map(|c| c.to_digit(10).map(|c| c as u8))
		.collect::<Option<Vec<_>>>()?;

	assert_eq!(data.len(), width*height);

	Some(Grid {data, width, height})
}


fn minimum_risk_factors_sum(grid: &Grid) -> usize {
	grid.iter_minima()
		.map(|(cell, ..)| cell as usize + 1)
		.sum()
}


fn basin_size(grid: &Grid, x: i32, y: i32) -> usize {
	let mut visited = HashSet::new();
	let mut queue = vec![(x, y)];

	while let Some((x, y)) = queue.pop() {
		visited.insert((x, y));

		let cell = grid.get(x, y).unwrap();

		let greater_neighbors = grid.iter_neighbors(x, y)
			.filter(|&(neighbor, x, y)| {
				neighbor > cell
				&& neighbor != 9
				&& !visited.contains(&(x, y))
			})
			.map(|(_, x, y)| (x, y));

		queue.extend(greater_neighbors);
	}

	visited.len()
}

fn largest_basin_sizes_product(grid: &Grid) -> usize {
	let mut basins = grid.iter_minima()
		.map(|(_, x, y)| basin_size(grid, x, y))
		.collect(): Vec<_>;

	basins.sort_by_key(|&v| -(v as i32));

	basins.iter().take(3)
		.fold(1, |acc, &size| acc * size)

}


#[test]
fn test_minimum_risk_factors_sum() {
	let grid = parse_grid(include_str!("day9-test.txt")).unwrap();
	let sum = minimum_risk_factors_sum(&grid);
	assert_eq!(sum, 15);
}

#[test]
fn test_basin_size() {
	let grid = parse_grid(include_str!("day9-test.txt")).unwrap();
	let tl_basin_size = basin_size(&grid, 1, 0);
	let tr_basin_size = basin_size(&grid, 9, 0);
	let middle_basin_size = basin_size(&grid, 2, 2);
	let br_basin_size = basin_size(&grid, 6, 4);

	assert_eq!(tl_basin_size, 3);
	assert_eq!(tr_basin_size, 9);
	assert_eq!(middle_basin_size, 14);
	assert_eq!(br_basin_size, 9);
}

#[test]
fn test_largest_basin_sizes_product() {
	let grid = parse_grid(include_str!("day9-test.txt")).unwrap();
	let product = largest_basin_sizes_product(&grid);

	assert_eq!(product, 1134);
}


#[test]
fn test_grid() {
	let grid = parse_grid(include_str!("day9-test.txt")).unwrap();

	assert_eq!(grid.get(1, 2), Some(8));
	assert_eq!(grid.get(0, 1), Some(3));

	assert_eq!(grid.iter_neighbors(0, 2).collect(): Vec<_>, &[
		(8, 1, 2),
		(3, 0, 1),
		(8, 0, 3),
	]);
}
