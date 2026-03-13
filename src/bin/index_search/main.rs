use std::io::{self, BufRead};
mod index_search;
#[path = "../util/screen.rs"]
mod screen;

/*
 * Complete the 'binarySearch' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY nums
 *  2. INTEGER target
 */

fn main() {
	screen::clear();
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let nums_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

	let mut nums: Vec<i32> = Vec::with_capacity(nums_count as usize);

	for _ in 0..nums_count {
		let nums_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
		nums.push(nums_item);
	}

	let target = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

	let result = index_search::binarySearch(&nums, target);

	println!("{}", result);
}
