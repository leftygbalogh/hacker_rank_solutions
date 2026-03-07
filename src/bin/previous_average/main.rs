mod prev_avg_proj_lib;
#[path = "../util/screen.rs"]
mod screen;
use std::io::{self, BufRead};

/*
 * Complete the 'count_response_time_regressions' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY responseTimes as parameter.
 * Manual input: first line: N number of entries; subsequent N lines, the actual entries
 * Run with
 * cargo run --bin previous_average
 */

fn main() {
	screen::clear();
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let response_times_count = stdin_iterator
		.next()
		.unwrap()
		.unwrap()
		.trim()
		.parse::<i32>()
		.unwrap();

	let mut response_times: Vec<i32> = Vec::with_capacity(response_times_count as usize);

	for _ in 0..response_times_count {
		let response_times_item: i32 = stdin_iterator
			.next()
			.unwrap()
			.unwrap()
			.trim()
			.parse::<i32>()
			.unwrap();
		response_times.push(response_times_item);
	}

	let result = prev_avg_proj_lib::count_response_time_regressions(&response_times);

	println!("{}", result);
}


/*
fn main(){
	screen::clear();
	proj_template::printer(43);
}

 */
