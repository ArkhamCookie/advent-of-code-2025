use common::input_file;

fn is_invalid(id: u64) -> bool {
	let digits = id.ilog10().div_ceil(2);
	let mask = u64::pow(10, digits);

	id % mask == id / mask
}

fn count_invaild(input: &str) -> u64 {
	let mut count = 0;

	for range in input.split(',') {
		let (start, end) = range.split_once('-').unwrap();
		let start: u64 = start.parse().unwrap();
		let end: u64 = end.parse().unwrap();

		for id in start..=end {
			count += is_invalid(id) as u64 * id;
		}
	}

	count
}

fn part_a(input: &str) -> u64 {
	count_invaild(input)
}

fn main() {
	let input = input_file();

	let part_a = part_a(&input);

	println!("Part A: {}", part_a);
}

#[cfg(test)]
mod tests {
    use crate::part_a;

	const IDS: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

	#[test]
	fn part_a_test() {
		assert_eq!(part_a(&IDS), 1227775554)
	}
}
