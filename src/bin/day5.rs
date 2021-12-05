#![feature(type_ascription, drain_filter)]



fn main() {
	let lines = include_str!("day5.txt").lines()
		.filter_map(parse_line)
		.collect(): Vec<_>;

	part_1(&lines);
	part_2(&lines);
}



fn part_1(lines: &[Line]) {
	let relevant_lines = lines.iter().cloned().filter(Line::is_orthogonal).collect(): Vec<_>;
	let mut chart = Chart::for_lines(&relevant_lines);

	for line in relevant_lines.iter() {
		chart.add_line(line);
	}

	println!("crossings: {}", chart.num_crossings());
}


fn part_2(lines: &[Line]) {
	let mut chart = Chart::for_lines(&lines);

	for line in lines.iter() {
		chart.add_line(line);
	}

	println!("crossings with diagonals: {}", chart.num_crossings());
}




#[derive(Debug, Copy, Clone)]
struct Line {
	start: Point,
	end: Point,
}

impl Line {
	fn is_orthogonal(&self) -> bool {
		self.start.x == self.end.x
		|| self.start.y == self.end.y
	}
}


#[derive(Debug, Copy, Clone)]
struct Point{ x: i32, y: i32 }


fn parse_line(s: &str) -> Option<Line> {
	let mut points = s.split(" -> ").map(parse_point);
	let start = points.next()??;
	let end = points.next()??;
	Some(Line{start, end})
}

fn parse_point(s: &str) -> Option<Point> {
	let mut points = s.split(',').filter_map(|s| s.parse().ok());
	let x = points.next()?;
	let y = points.next()?;
	Some(Point{x, y})
}



#[derive(Debug)]
struct Chart {
	data: Vec<i32>,
	width: usize,
}

impl Chart {
	fn for_lines(lines: &[Line]) -> Chart {
		let (max_x, max_y) = lines.iter()
			.flat_map(|Line{start, end}| [start, end])
			.fold((0, 0), |(mxx, mxy), &Point{x, y}| {
				(mxx.max(x as usize), mxy.max(y as usize))
			});

		let (width, height) = (max_x+1, max_y+1);

		let size = width*height;
		let data = vec![0; size];

		Chart {data, width}
	}

	fn add_line(&mut self, Line{start, end}: &Line) {
		let dx = end.x - start.x;
		let dy = end.y - start.y;

		let dir_x = dx.signum();
		let dir_y = dy.signum();

		if dx != 0 && dy != 0 {
			assert!(dx.abs() == dy.abs());
		}

		let distance = dx.abs().max(dy.abs());
		for i in 0..=distance {
			let p = Point{
				x: start.x + i * dir_x,
				y: start.y + i * dir_y
			};

			self.add_point(p);
		}
	}

	fn add_point(&mut self, Point{x, y}: Point) {
		self.data[x as usize + self.width * y as usize] += 1;
	}

	fn num_crossings(&self) -> usize {
		self.data.iter().filter(|&&cell| cell > 1).count()
	}
}