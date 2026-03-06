pub fn countResponseTimeRegressions(responseTimes: &[i32]) -> i32 {
    // Write your code here
    //println!("{:?}", &responseTimes);

    let mut divisor: u64 = 0;
    let mut counter = 0;
    let mut subtotal: u64 = 0;

    for entry in responseTimes {
        subtotal += *entry as u64;
        divisor += 1;
        if *entry as u64 > (subtotal / divisor) {
            counter += 1;
        }
    }
    counter
}

//Test with
//cargo test --bin previous_average
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
    fn test_1() {
        assert_eq!(countResponseTimeRegressions(&[0]), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(countResponseTimeRegressions(&[4, 5, 6]), 2);
        assert_eq!(countResponseTimeRegressions(&[100, 200, 150, 300, 300]), 3);
		assert_ne!(countResponseTimeRegressions(&[42,31,42,31,42]), 1);
    }
}
