mod smallest_int_lib;

#[path = "../util/screen.rs"]
mod screen;
use std::io::{self, BufRead};

fn main() {
	println!("\x1b[2J\x1b[H\x1b[3J");
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let order_numbers_count = stdin_iterator
		.next()
		.unwrap()
		.unwrap()
		.trim()
		.parse::<i64>()
		.unwrap();

	let mut order_numbers: Vec<i64> = Vec::with_capacity(order_numbers_count as usize);

	for _ in 0..order_numbers_count {
		let order_numbers_item = stdin_iterator
			.next()
			.unwrap()
			.unwrap()
			.trim()
			.parse::<i64>()
			.unwrap();
		order_numbers.push(order_numbers_item);
	}

	let result = smallest_int_lib::find_smallest_missing_positive(&order_numbers);

	println!("{}", result);
}