use super::Day;
use itertools::Itertools;
use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let ord_rules = input
		.lines()
		.take_while(|l| l.len() > 1)
		.map(|l| l.split('|').map(|x| x.parse().unwrap()).collect_tuple().unwrap())
		.into_grouping_map()
		.collect();

	let updates = input
		.lines()
		.skip_while(|l| l.len() > 1)
		.skip(1)
		.map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
		.collect();

	Box::new(Day5 { ord_rules, updates })
}

struct Day5 {
	ord_rules: HashMap<i32, HashSet<i32>>,
	updates: Vec<Vec<i32>>,
}
impl Day5 {
	fn is_ordered(&self, update: &[i32]) -> bool {
		update
			.iter()
			.tuple_windows()
			.all(|(x, y)| self.ord_rules.get(x).and_then(|f| f.get(y)).is_some())
	}
}
impl Day for Day5 {
	fn part1(&self) -> i32 {
		self.updates
			.iter()
			.filter(|update| self.is_ordered(update))
			.map(|update| update[update.len() / 2])
			.sum()
	}

	fn part2(&self) -> i32 {
		self.updates
			.iter()
			.filter(|update| !self.is_ordered(update))
			.map(|update| {
				let mut update = update.clone();
				update.sort_by(|a, b| {
					self.ord_rules
						.get(a)
						.and_then(|f| f.get(b))
						.map_or(Ordering::Less, |_| Ordering::Greater)
				});
				update[update.len() / 2]
			})
			.sum()
	}
}
