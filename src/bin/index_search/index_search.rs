pub fn binarySearch(nums: &[i32], target: i32) -> i32 {
	//default_solution(nums, &target)
	let mut true_index = (nums.len()/2) as i32;
	let (nums , target, true_index) = recursiveBinarySearch(nums, target, true_index);
	println!("nums: {:?} target{} true index {}", nums, target, true_index);




	42


}

fn recursiveBinarySearch(nums: &[i32], target: i32, true_index: i32) -> (&mut [i32], i32, i32) {
	if nums.len() == 1 {
		if nums[0] == target {
			let x = (nums, target, true_index);
			return x
		}
		else { return (nums, 0, -1) }
		}

	else {
		let head: &mut[i32];
		let tail: &mut[i32];
		let mut length = nums.len()/2;
		let true_index = true_index  + length as i32;
		if nums[0] < target {
			(head, tail) = nums.split_at_mut(length);
			return recursiveBinarySearch(head, target, true_index);
		}
		else {
			(head, tail) = nums.split_at_mut(length);
			return recursiveBinarySearch(tail, target, true_index) }
	} }


fn default_solution(nums: &[i32], target: &i32) -> i32 {
	let result = nums.binary_search(&target);
	if result.is_ok() {
		result.unwrap() as i32
	} else {
		-1
	}
}