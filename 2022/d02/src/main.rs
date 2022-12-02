use std::{
	ops::Add,
	str::FromStr,
};

#[derive(Debug)]
#[repr(usize)]
enum Hand {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

#[derive(Debug)]
#[repr(usize)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

fn main() {
	let result = include_str!("../input.txt").split('\n').fold(
		(0, 0),
		|accumulated: (usize, usize), element: &str| {
			let (other_str, me_str) = element.split_once(' ').unwrap();
			let me = me_str.parse::<Hand>().unwrap();
			let result = me_str.parse::<Outcome>().unwrap();
			let other = other_str.parse::<Hand>().unwrap();
			let result_part_1 = calc_part_1(&me, &other);
			let result_part_2 = calc_part_2(&result, &other);

			(
				accumulated.0 + (result_part_1 + me),
				accumulated.1 + (result_part_2 + result),
			)
		},
	);
	println!("Part 1: {}", result.0);
	println!("Part 2: {}", result.1);
}

impl FromStr for Hand {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(Hand::Rock),
			"B" | "Y" => Ok(Hand::Paper),
			"C" | "Z" => Ok(Hand::Scissors),
			_ => unreachable!(),
		}
	}
}

impl FromStr for Outcome {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Outcome::Loss),
			"Y" => Ok(Outcome::Draw),
			"Z" => Ok(Outcome::Win),
			_ => unreachable!(),
		}
	}
}

impl Add<Outcome> for Hand {
	type Output = usize;
	fn add(self, other: Outcome) -> usize {
		self as usize + other as usize
	}
}

impl Add<Hand> for Outcome {
	type Output = usize;
	fn add(self, other: Hand) -> usize {
		self as usize + other as usize
	}
}

fn calc_part_1(me: &Hand, other: &Hand) -> Outcome {
	match me {
		Hand::Rock => match other {
			Hand::Rock => Outcome::Draw,
			Hand::Paper => Outcome::Loss,
			Hand::Scissors => Outcome::Win,
		},
		Hand::Paper => match other {
			Hand::Rock => Outcome::Win,
			Hand::Paper => Outcome::Draw,
			Hand::Scissors => Outcome::Loss,
		},
		Hand::Scissors => match other {
			Hand::Rock => Outcome::Loss,
			Hand::Paper => Outcome::Win,
			Hand::Scissors => Outcome::Draw,
		},
	}
}

fn calc_part_2(result: &Outcome, other: &Hand) -> Hand {
	match result {
		Outcome::Win => match other {
			Hand::Rock => Hand::Paper,
			Hand::Paper => Hand::Scissors,
			Hand::Scissors => Hand::Rock,
		},
		Outcome::Draw => match other {
			Hand::Rock => Hand::Rock,
			Hand::Paper => Hand::Paper,
			Hand::Scissors => Hand::Scissors,
		},
		Outcome::Loss => match other {
			Hand::Rock => Hand::Scissors,
			Hand::Paper => Hand::Rock,
			Hand::Scissors => Hand::Paper,
		},
	}
}
