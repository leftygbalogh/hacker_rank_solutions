mod palindrome;
#[path = "../util/screen.rs"]
mod screen;
use std::io::{self, BufRead};
//use hackerrank_solutions::palindrome;
/*
 * Complete the 'is_alphabetic_palindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */

//cargo run --bin nontrivial_palindrome
fn main() {
	screen::clear();
	println!("nontrivial palindrome");
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let code = stdin_iterator.next().unwrap().unwrap();

	let result = palindrome::is_alphabetic_palindrome(&code);

	println!("{}", if result { 1 } else { 0 });
}