use common::input_file;

fn part_a(input: &str) -> i32 {
	let mut position = 50;
	let mut output = 0;

	for line in input.lines() {
		let line = line.trim();

		if line.is_empty() {
			continue;
		}

		let number: i32 = line[1..].parse().unwrap();
		match line.as_bytes()[0] {
			b'L' => position = (100 + position - number % 100) % 100,
			b'R' => position = (position + number) % 100,
			_ => unreachable!("unexpected instruction"),
		}

		if position == 0 {
			output += 1;
		}
	}

	output
}

fn part_b(input: &str) -> i32 {
	let mut position = 50;
	let mut output = 0;

	for line in input.lines() {
		let line = line.trim();

		if line.is_empty() {
			continue;
		}

		let number: i32 = line[1..].parse().unwrap();
		match line.as_bytes()[0] {
			b'L' => {
				output += (99 - (position + 99) % 100 + number) / 100;
				position = (100 + position - number % 100) % 100
			}
			b'R' => {
				output += (position + number) / 100;
				position = (position + number) % 100
			}
			_ => unreachable!("unexpected instruction"),
		}
	}

	output
}

fn main() {
	let input = input_file();

	let part_a = part_a(&input);
	let part_b = part_b(&input);

	println!("Part A: {}", part_a);
	println!("Part B: {}", part_b)
}

#[cfg(test)]
mod tests {
	use super::{part_a, part_b};

	use indoc::indoc;

    const CASE: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

	#[test]
	fn part_a_test() {
		assert_eq!(part_a(&CASE), 3);
	}

	#[test]
	fn part_b_test() {
		assert_eq!(part_b(&CASE), 6);
	}
}
