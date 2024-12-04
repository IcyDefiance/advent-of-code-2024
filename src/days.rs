mod day1;
mod day2;
mod day3;

use chrono::{TimeDelta, Utc};
use console::style;
use humanize_duration::{Truncate, prelude::DurationExt};
use lazy_static::lazy_static;
use std::borrow::Cow;

use crate::progress::using_spinner;

pub trait Day {
	fn part1(&self) -> i32;
	fn part2(&self) -> i32;
}

lazy_static! {
	static ref DAYS: Vec<fn(&str) -> Box<dyn Day>> = vec![day1::preprocess, day2::preprocess, day3::preprocess];
}

pub fn run_day(day: usize, input: &str) {
	let (day, time_delta) = using_timed_spinner("Pre-processing input...", || DAYS[day - 1](input));
	println!(
		"{} Input pre-processed (took {})",
		style("✔").green(),
		time_delta.human(Truncate::Nano)
	);

	let (answer, time_delta) = using_timed_spinner("Running part 1...", || day.part1());
	println!(
		"🎯 Part 1 answer: {} (took {})",
		answer,
		time_delta.human(Truncate::Nano)
	);

	let (answer, time_delta) = using_timed_spinner("Running part 2...", || day.part2());
	println!(
		"🎯 Part 2 answer: {} (took {})",
		answer,
		time_delta.human(Truncate::Nano)
	);
}

fn using_timed_spinner<T>(message: impl Into<Cow<'static, str>>, f: impl FnOnce() -> T) -> (T, TimeDelta) {
	using_spinner(message, || {
		let start = Utc::now();
		let res = f();
		let end = Utc::now();
		(res, end - start)
	})
}
