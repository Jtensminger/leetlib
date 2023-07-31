/* 
Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
If target is not found in the array, return [-1, -1]. You must write an algorithm with O(log n) runtime complexity.
*/
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
                Ok(_) => vec![nums.partition_point(|&x| x < target) as i32, nums.partition_point(|&x| x <= target) as i32 - 1],
                Err(_) => vec![-1, -1]
        }
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(vec![3,4], search_range(vec![5,7,7,8,8,10], 8));
                //assert_eq!(vec![-1,-1], search_range(vec![5,7,7,8,8,10], 6));
                //assert_eq!(vec![-1,-1], search_range(vec![], 0));
        }
}
