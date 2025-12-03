use common::input_file;

fn joltage(digits: &[u8], power_size: usize) -> u64 {
	let mut number = 0;
	let mut last_used = 0;

	for battery in 0..power_size {
		let size = &digits.len() - (power_size - 1 - battery) - last_used;
		let slice = &digits[last_used..(last_used + size).min(digits.len())];

		for i in (0..=9).rev() {
			if let Some(idx) = slice.iter().position(|x| *x == i) {
				last_used += idx + 1;
				number = number * 10 + i as u64;
				break;
			}
		}
	}

	number
}

fn part_a(input: &str) -> u64 {
	let mut output = 0;

	for bank in input.trim().lines() {
		let digits = bank
			.chars()
			.map(|x| x.to_digit(10).unwrap() as u8)
			.collect::<Vec<_>>();

		output += joltage(&digits, 2)
	}

	output
}

fn part_b(input: &str) -> u64 {
	let mut output = 0;

	for bank in input.trim().lines() {
		let digits = bank
			.chars()
			.map(|x| x.to_digit(10).unwrap() as u8)
			.collect::<Vec<_>>();

		output += joltage(&digits, 12)
	}

	output
}

fn main() {
	let input = input_file();

	let part_a = part_a(&input);
	let part_b = part_b(&input);

	println!("Part A: {}", part_a);
	println!("Part B: {}", part_b);
}

#[cfg(test)]
mod tests {
	use super::{part_a, part_b};

	use indoc::indoc;

	const CASE: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

	#[test]
	fn part_a_test() {
		assert_eq!(part_a(&CASE), 357)
	}

	#[test]
	fn part_b_test() {
		assert_eq!(part_b(&CASE), 3121910778619)
	}
}
