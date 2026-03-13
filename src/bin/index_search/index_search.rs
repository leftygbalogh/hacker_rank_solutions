pub fn binarySearch(nums: &[i32], target: i32) -> i32 {
	//default_solution(nums, &target)

	let mut length = nums.len();
	let mut index = length / 2;
	if let Some(value) = is_value_at_index(nums, target, &mut index) {
		return value;
	}
	else {
		if  {  }
	}
	if nums[index] < target {
		length = index / 2;
		index = index + length;
		if nums[index] == target {
			return index as i32;
		}

	}

	42


}

fn is_value_at_index(nums: &[i32], target: i32, index: &mut usize) -> Option<i32> {
	if nums[index] == target {
		return Some(true)
	} else {
		return Some(false)
	}
	None
}

fn default_solution(nums: &[i32], target: &i32) -> i32 {
	let result = nums.binary_search(&target);
	if result.is_ok() {
		result.unwrap() as i32
	} else {
		-1
	}
}