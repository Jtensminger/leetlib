// Dynamic Programming
/* Problem Statement:
	Given an integer array nums, 
	find the subarray with the largest sum, and return its sum.
*/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
	let mut max_sum = nums[0];
	let mut window_sum = 0;
	
	for i in 0..nums.len() {
		window_sum = window_sum.max(0);
		window_sum += nums[i];
		max_sum = max_sum.max(window_sum);
	}

	max_sum
}

mod test {
	use super::*;

	#[test]
	fn ext1() {
		let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
		assert_eq!(6, max_sub_array(nums));
	}

	#[test]
	fn ext2() {
		let nums = vec![1];
		assert_eq!(1, max_sub_array(nums));
	}

	#[test]
	fn ext3() {
		let nums = vec![5,4,-1,7,8];
		assert_eq!(23, max_sub_array(nums));
	}

	#[test]
	fn ext4() {
		let nums = vec![-6,0,1,-2];
		assert_eq!(1, max_sub_array(nums));
	}
}




/*
OLD INITIAL THOUGHTS:
	sub-problems: 
		find all sub_arrays: elems >= 1
		sum each of them
		return the max sub_array


			  	  28=[5,4,-1,7,8]
	     15=[5,4,-1,7] 			  	18=[4,-1,7,8]

        8=[5,4,-1]        10=[4,-1,7]	    	  10=[4,-1,7]    	15=[-1,7,8]

     9=[5,4] 3=[4,-1]  3=[4,-1] 6=[-1,7] 	3=[4,-1] 6=[-1,7]  	6=[-1,7] 15=[7,8]
 */