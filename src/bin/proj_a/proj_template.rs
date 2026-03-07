use rand::prelude::ThreadRng;

pub fn printer(input: i32) -> i32 {
	print!("{}", &input);
	input
}

//cargo test --bin proj_a
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_1(){
		assert_eq!(printer(1),1);
	}

}