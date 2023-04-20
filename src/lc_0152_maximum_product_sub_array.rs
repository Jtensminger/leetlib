/*
Given an integer array nums, 
find a subarray that has the largest product, and return the product.
Constraints:
	1 <= nums.length <= 2 * 104
	-10 <= nums[i] <= 10
	The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
*/

use std::mem::swap;


pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = nums[0]; 
	let mut max_window = 1;
	let mut min_window = 1;

	for n in nums {
		if n < 0 {
			swap(&mut max_window, &mut min_window)
		}
		max_window = n.max(max_window * n);
		min_window = n.min(min_window * n);
		max_product = max_product.max(max_window);
	}
	max_product
}

mod test {
	use super::*;

	#[test]
	fn ext1() {
		let nums = vec![2,3,-2,4];
		assert_eq!(6, max_product(nums));
		// [2,3] has the largest product 6.
	}

	#[test]
	fn ext2() {
		let nums = vec![-2,0,-1];
		assert_eq!(0, max_product(nums));
		// The result cannot be 2, because [-2,-1] is not a subarray.
	}

	#[test]
	fn ext3() {
		let nums = vec![-3,-1,-1];
		assert_eq!(3, max_product(nums));
	}

	#[test]
	fn ext4() {
		let nums = vec![-4, -3];
		assert_eq!(12, max_product(nums));
	}

	#[test]
	fn ext5() {
		let nums = vec![0, 2];
		assert_eq!(2, max_product(nums));
	}
}
