// Dynamic Programming
/* Problem Statement:
	Given an integer array nums, 
	find the subarray with the largest sum, and return its sum.
*/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
	let mut max_sum = nums[0];
	let mut window_sum = 0;
	
	for i in 0..nums.len() {
		window_sum = window_sum.max(0) + nums[i];
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
Key Insights:
When solving the Maximum Subarray problem, several key insights can help you find an optimal solution:

1) Subarray continuity:
	The subarray must be contiguous, meaning you can't skip any elements within the subarray.
	This property is crucial when devising a solution because it constrains the possibilities you need to explore.

2) Local vs. global maxima:
	The problem asks for a global maximum subarray sum, which means the largest sum across all possible subarrays.
	However, it's helpful to consider the local maximum subarray sum as you iterate through the array.
	This insight is the basis for Kadane's algorithm, which compares local maxima at each step to find the global maximum.

3) Optimal substructure:
	The Maximum Subarray problem has an optimal substructure, meaning the solution can be built from optimal solutions to overlapping subproblems.
	This property allows you to use dynamic programming approaches, such as memoization and bottom-up algorithms, to find the optimal solution efficiently.

4) Divide and conquer:
	The problem can be solved using a divide-and-conquer strategy by recursively breaking the problem down into smaller subproblems.
	This approach is less efficient than Kadane's algorithm but helps to illustrate how the problem can be solved using different techniques.


 */


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