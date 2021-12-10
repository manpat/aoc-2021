#![feature(type_ascription)]

fn main() {
	let entries = include_str!("day8.txt").lines()
		.filter_map(parse_entry)
		.collect(): Vec<_>;

	dbg!(count_unique_segment_output_digits(&entries));
	dbg!(calculate_entry_sum(&entries));
}


#[derive(Debug)]
struct Entry {
	inputs: Vec<Glyph>,
	outputs: Vec<Glyph>,
}

#[derive(Debug, Clone, Copy)]
struct Glyph {
	// bitset - each bit assigned to a segment
	segments: u8
}

impl Glyph {
	fn active_segment_count(&self) -> u32 {
		self.segments.count_ones()
	}

	fn xor(&self, o: &Glyph) -> Glyph {
		Glyph {
			segments: self.segments ^ o.segments
		}
	}

	fn contains(&self, pattern: &Glyph) -> bool {
		self.segments & pattern.segments == pattern.segments
	}
}


fn parse_entry(s: &str) -> Option<Entry> {
	let mut parts_it = s.split('|');

	let mut inputs = parts_it.next().and_then(parse_glyph_sequence)?;
	let outputs = parts_it.next().and_then(parse_glyph_sequence)?;

	// Order of inputs doesn't matter - so sort them into a more convenient order
	inputs.sort_by_key(Glyph::active_segment_count);

	Some(Entry{inputs, outputs})
}

fn parse_glyph_sequence(s: &str) -> Option<Vec<Glyph>> {
	s.split_ascii_whitespace()
		.map(parse_glyph)
		.collect()
}

fn parse_glyph(s: &str) -> Option<Glyph> {
	let mut segments = 0u8;

	for c in s.trim().chars() {
		assert!(('a'..='g').contains(&c));

		let index = c.to_digit(17)?
			.checked_sub(10)?;

		segments |= 1 << index;
	}

	Some(Glyph{segments})
}




fn count_unique_segment_output_digits(entries: &[Entry]) -> usize {
	let unique_segment_counts = [2, 3, 4, 7];

	entries.iter()
		.flat_map(|entry| &entry.outputs)
		.map(Glyph::active_segment_count)
		.filter(|seg_count| unique_segment_counts.contains(seg_count))
		.count()
}



//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg


fn decode_entry_value(entry: &Entry) -> usize {
	// order: 1 7 4 x x x x x x 8
	let inputs = &entry.inputs;
	assert!(inputs.len() == 10);

	let cf_seg = inputs[0];
	let bcdf_seg = inputs[2];
	let bd_seg = cf_seg.xor(&bcdf_seg);

	let get_value = |glyph: &Glyph| {
		match glyph.active_segment_count() {
			2 => 1,
			3 => 7,
			4 => 4,
			5 => {
				if glyph.contains(&cf_seg) {
					3
				} else if glyph.contains(&bd_seg) {
					5
				} else {
					2
				}
			}
			6 => {
				if !glyph.contains(&cf_seg) {
					6
				} else if glyph.contains(&bd_seg) {
					9
				} else {
					0
				}
			}
			7 => 8,

			_ => unreachable!(),
		}
	};

	entry.outputs.iter()
		.map(get_value)
		.fold(0, |acc, digit| acc * 10 + digit)
}


fn calculate_entry_sum(entries: &[Entry]) -> usize {
	entries.iter().map(decode_entry_value).sum()
}





#[test]
fn test_count_unique_segment_output_digits() {
	let entries = include_str!("day8-test.txt").lines()
		.map(parse_entry)
		.collect::<Option<Vec<_>>>()
		.unwrap();

	let count = count_unique_segment_output_digits(&entries);

	assert_eq!(count, 26);
}


#[test]
fn test_decode_entry_value() {
	let entry = parse_entry("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
		.unwrap();

	let value = decode_entry_value(&entry);

	assert_eq!(value, 5353);



	let entries = include_str!("day8-test.txt").lines()
		.map(parse_entry)
		.collect::<Option<Vec<_>>>()
		.unwrap();

	let expected_values = [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

	for (entry, &expected) in entries.iter().zip(&expected_values) {
		let value = decode_entry_value(entry);
		assert_eq!(value, expected);
	}
}


#[test]
fn test_calculate_entry_sum() {
	let entries = include_str!("day8-test.txt").lines()
		.map(parse_entry)
		.collect::<Option<Vec<_>>>()
		.unwrap();

	let sum = calculate_entry_sum(&entries);

	assert_eq!(sum, 61229);
}