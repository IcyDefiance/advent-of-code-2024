mod days;
mod progress;

use anyhow::Result;
use clap::Parser;
use console::style;
use days::run_day;
use progress::using_spinner;
use std::fs;

#[derive(Parser)]
struct Args {
	/// The problem to solve
	day: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();

	let input = get_input(args.day).await?;

	run_day(args.day, &input);

	Ok(())
}

async fn get_input(day: usize) -> Result<String> {
	let input = using_spinner("Loading input...", || {
		fs::read_to_string(format!("inputs/day{}.txt", day))
	})?;
	println!("{} Input loaded", style("✔").green());
	Ok(input)
}
