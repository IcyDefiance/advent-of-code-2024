use super::Day;
use nom::{
	IResult,
	branch::alt,
	bytes::complete::tag,
	character::complete::{anychar, char, i32},
	combinator::map,
	multi::{many_till, many1},
	sequence::{delimited, separated_pair},
};

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let (_, instructions) = program(input).unwrap();
	Box::new(Day3 { instructions })
}

struct Day3 {
	instructions: Vec<Instruction>,
}
impl Day for Day3 {
	fn part1(&self) -> i32 {
		self.instructions
			.iter()
			.map(|inst| match inst {
				Instruction::Mul(num1, num2) => num1 * num2,
				_ => 0,
			})
			.sum()
	}

	fn part2(&self) -> i32 {
		let mut enabled = true;
		let mut sum = 0;

		for inst in &self.instructions {
			match inst {
				Instruction::Do => enabled = true,
				Instruction::Dont => enabled = false,
				Instruction::Mul(num1, num2) => {
					if enabled {
						sum += num1 * num2;
					}
				}
			}
		}

		sum
	}
}

enum Instruction {
	Mul(i32, i32),
	Do,
	Dont,
}

fn program(input: &str) -> IResult<&str, Vec<Instruction>> {
	let inst_mul = map(
		delimited(tag("mul("), separated_pair(i32, char(','), i32), char(')')),
		|(num1, num2)| Instruction::Mul(num1, num2),
	);
	let inst_do = map(tag("do()"), |_| Instruction::Do);
	let inst_dont = map(tag("don't()"), |_| Instruction::Dont);

	let instruction = alt((inst_mul, inst_do, inst_dont));

	let mut program = many1(map(many_till(anychar, instruction), |(_, inst)| inst));

	program(input)
}
