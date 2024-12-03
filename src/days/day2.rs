use super::Day;
use itertools::Itertools;

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let reports = input.lines().map(split_and_parse_line).collect();
	Box::new(Day2 { reports })
}

struct Day2 {
	reports: Vec<Vec<i32>>,
}
impl Day for Day2 {
	fn part1(&self) -> i32 {
		self.reports.iter().filter(|&x| is_safe(x)).count() as _
	}

	fn part2(&self) -> i32 {
		self.reports.iter().filter(|&x| is_safe_dampened(x)).count() as _
	}
}

fn split_and_parse_line(line: &str) -> Vec<i32> {
	line.split(" ").map(|x| x.parse().unwrap()).collect()
}

fn is_safe(sequence: &[i32]) -> bool {
	let increasing = sequence[0] < sequence[1];
	sequence
		.iter()
		.tuple_windows()
		.all(|(a, b)| if increasing { a < b } else { a > b } && (a - b).abs() <= 3)
}

fn is_safe_dampened(sequence: &[i32]) -> bool {
	if is_safe(sequence) {
		return true;
	}

	(0..sequence.len()).any(|i| {
		let mut sequence = sequence.to_vec();
		sequence.remove(i);
		is_safe(&sequence)
	})
}
