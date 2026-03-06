mod palindrome;

use std::io::{self, BufRead};
//use hackerrank_solutions::palindrome;
/*
 * Complete the 'isAlphabeticPalindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */

//cargo run --bin nontrivial_palindrome
fn main() {
	println!("\x1b[2J\x1b[H\x1b[3J");
	println!("nontrivial palindrome");
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let code = stdin_iterator.next().unwrap().unwrap();

	let result = palindrome::isAlphabeticPalindrome(&code);

	println!("{}", if result { 1 } else { 0 });
}