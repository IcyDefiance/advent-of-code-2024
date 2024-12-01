use super::Day;
use itertools::Itertools;
use std::{collections::HashMap, iter::zip};

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let (mut left, mut right): (Vec<_>, Vec<_>) = input.lines().map(split_and_parse_line).unzip();

	left.sort();
	right.sort();

	Box::new(Day1 { left, right })
}

struct Day1 {
	left: Vec<i32>,
	right: Vec<i32>,
}
impl Day for Day1 {
	fn part1(&self) -> i32 {
		zip(&self.left, &self.right).map(|(l, r)| (l - r).abs()).sum()
	}

	fn part2(&self) -> i32 {
		let right_freq: HashMap<_, _> = self
			.right
			.iter()
			.chunk_by(|&&x| x)
			.into_iter()
			.map(|(k, g)| (k, g.count() as i32))
			.collect();

		self.left
			.iter()
			.map(|&x| x * right_freq.get(&x).cloned().unwrap_or(0))
			.sum()
	}
}

fn split_and_parse_line(line: &str) -> (i32, i32) {
	line.split("   ").map(|x| x.parse().unwrap()).collect_tuple().unwrap()
}
