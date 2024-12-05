use super::Day;
use array_init::array_init;
use ascii::{AsciiChar, AsciiStr, AsciiString};

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let lines = input
		.lines()
		.map(|line| AsciiString::from_ascii(line).unwrap())
		.collect();
	Box::new(Day4 { lines })
}

struct Day4 {
	lines: Vec<AsciiString>,
}
impl Day4 {
	fn count_xmas(&self, x: usize, y: usize) -> i32 {
		let xmas = AsciiStr::from_ascii("XMAS").unwrap();
		let samx = AsciiStr::from_ascii("SAMX").unwrap();

		let can_increase_x = x + 3 < self.lines[0].len();
		let can_increase_y = y + 3 < self.lines.len();

		let horizontal: Option<[AsciiChar; 4]> = can_increase_x.then(|| array_init(|i| self.lines[y][x + i]));
		let vertical: Option<[AsciiChar; 4]> = can_increase_y.then(|| array_init(|i| self.lines[y + i][x]));
		let diagonal1: Option<[AsciiChar; 4]> =
			(can_increase_x && can_increase_y).then(|| array_init(|i| self.lines[y + i][x + i]));
		let diagonal2: Option<[AsciiChar; 4]> =
			(can_increase_x && can_increase_y).then(|| array_init(|i| self.lines[y + i][x + 3 - i]));

		[horizontal, vertical, diagonal1, diagonal2]
			.iter()
			.flatten()
			.filter(|&string| (string[..] == *xmas || string[..] == *samx))
			.count() as i32
	}

	fn is_x_mas(&self, x: usize, y: usize) -> bool {
		let mas = AsciiStr::from_ascii("MAS").unwrap();
		let sam = AsciiStr::from_ascii("SAM").unwrap();

		let diagonal1: [AsciiChar; 3] = array_init(|i| self.lines[y - 1 + i][x - 1 + i]);
		let diagonal2: [AsciiChar; 3] = array_init(|i| self.lines[y - 1 + i][x + 1 - i]);

		(diagonal1[..] == *mas || diagonal1[..] == *sam) && (diagonal2[..] == *mas || diagonal2[..] == *sam)
	}
}
impl Day for Day4 {
	fn part1(&self) -> i32 {
		(0..self.lines.len())
			.flat_map(|y| (0..self.lines[0].len()).map(move |x| self.count_xmas(x, y)))
			.sum()
	}

	fn part2(&self) -> i32 {
		(1..self.lines.len() - 1)
			.flat_map(|y| (1..self.lines[0].len() - 1).filter(move |&x| self.is_x_mas(x, y)))
			.count() as i32
	}
}
