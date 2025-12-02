use std::env;
use std::fs;
use std::process::exit;

pub fn input_file() -> String {
	let path = match env::args().nth(1) {
		Some(arg) => arg,
		None => {
			eprintln!("Missing required arguement: <input-path>");
			exit(1)
		}
	};

	match fs::read_to_string(path) {
		Ok(file) => file,
		Err(error) => {
			eprintln!("{}", error);
			exit(1);
		}
	}
}
