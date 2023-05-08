/* 
Problem:
	Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
	Note that you must do this in-place without making a copy of the array.
Constraints:
	1 <= nums.length <= 104
	-231 <= nums[i] <= 231 - 1
*/
pub fn move_zeroes(nums: &mut Vec<i32>) {
	let l = nums.len();
	nums.retain(|&x| x != 0);
	nums.resize_with(l, || 0);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ext1() {
		// ext1
		let mut nums = vec![0, 1, 0, 3, 12];
		let target = vec![1, 3, 12, 0, 0];
		move_zeroes(&mut nums);
		assert_eq!(target, nums);
		
		// ext2
		let mut nums = vec![0];
		let target = vec![0];
		move_zeroes(&mut nums);
		assert_eq!(target, nums);

	}
}
