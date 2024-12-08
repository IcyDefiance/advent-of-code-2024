use super::Day;
use enumflags2::{BitFlags, bitflags};

pub fn preprocess(input: &str) -> Box<dyn Day> {
	let mut map = vec![];
	let mut guard_pos = (0, 0);

	for (y, line) in input.lines().enumerate() {
		let mut row = vec![];
		for (x, c) in line.chars().enumerate() {
			match c {
				'.' => row.push(MapTile::Empty),
				'#' => row.push(MapTile::Wall),
				'^' => {
					row.push(MapTile::Visited(Direction::North.into()));
					guard_pos = (x, y);
				}
				x => panic!("Invalid character in input: {}", x),
			}
		}
		map.push(row);
	}

	Box::new(Day6 {
		map,
		guard_pos,
		guard_facing: Direction::North,
	})
}

struct Day6 {
	map: Vec<Vec<MapTile>>,
	guard_pos: (usize, usize),
	guard_facing: Direction,
}
impl Day6 {
	fn play(&self, map: Vec<Vec<MapTile>>) -> Option<Vec<Vec<MapTile>>> {
		let mut map = map.clone();
		let mut pos = self.guard_pos;
		let mut facing = self.guard_facing;

		loop {
			let mut next_pos = pos;
			match facing {
				Direction::North => next_pos.1 = next_pos.1.wrapping_sub(1),
				Direction::East => next_pos.0 += 1,
				Direction::South => next_pos.1 += 1,
				Direction::West => next_pos.0 = next_pos.0.wrapping_sub(1),
			}

			if next_pos.0 >= map[0].len() || next_pos.1 >= map.len() {
				// End of map
				break;
			} else if map[next_pos.1][next_pos.0] == MapTile::Wall {
				// Turn right
				facing = match facing {
					Direction::North => Direction::East,
					Direction::East => Direction::South,
					Direction::South => Direction::West,
					Direction::West => Direction::North,
				};
			} else {
				// Move forward
				pos = next_pos;
				if map[pos.1][pos.0].is_visited_facing(facing) {
					return None;
				}
				map[pos.1][pos.0].set_visited(facing);
			}
		}

		Some(map)
	}
}
impl Day for Day6 {
	fn part1(&self) -> i32 {
		self.play(self.map.clone())
			.unwrap()
			.iter()
			.map(|row| row.iter().filter(|&&t| t.is_visited()).count())
			.sum::<usize>() as i32
	}

	fn part2(&self) -> i32 {
		let mut answer = 0;
		for y in 0..self.map.len() {
			for x in 0..self.map[0].len() {
				if self.map[y][x] == MapTile::Wall || (x, y) == self.guard_pos {
					continue;
				}

				let mut map = self.map.clone();
				map[y][x] = MapTile::Wall;

				if self.play(map).is_none() {
					answer += 1;
				}
			}
		}
		answer
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MapTile {
	Empty,
	Wall,
	Visited(BitFlags<Direction>),
}
impl MapTile {
	fn is_visited(&self) -> bool {
		match self {
			MapTile::Visited(_) => true,
			_ => false,
		}
	}

	fn is_visited_facing(&self, dir: Direction) -> bool {
		match self {
			MapTile::Visited(direction) => direction.contains(dir),
			_ => false,
		}
	}

	fn set_visited(&mut self, dir: Direction) {
		match *self {
			MapTile::Empty => *self = MapTile::Visited(dir.into()),
			MapTile::Wall => panic!("Cannot visit a wall"),
			MapTile::Visited(direction) => *self = MapTile::Visited(direction | dir),
		}
	}
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
	North = 0b0001,
	East = 0b0010,
	South = 0b0100,
	West = 0b1000,
}
