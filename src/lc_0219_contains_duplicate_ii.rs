/* 
Given an integer array nums and an integer k,
return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

"two distinct indices i and j in the array such that nums[i] == nums[j]" -> question: is duplicate?
"abs(i - j) <= k" -> constraint: <= k slots away from each other
*/

/* 
        nums[i] == nums[j] -> duplicate #     -> Decision Problem
        abs(i - j) <= k    -> max_window_size -> Dynamic Sliding Window
        rephrased as: find a number whose duplicate is <= k spots away or return false
        It's a decision problem, which means it's possible to short-circuit the algo if we find it because searching the whole space.
*/
use std::collections::HashSet;
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let (mut start, mut end) = (0, 0);
        while end != nums.len() - 1 { // search for start until breaking max_window_size: then increment start
                if end - start <= k as usize - 1 {
                        end += 1;
                } else {
                        start += 1;
                }
                if nums[start] == nums[end] {
                        return true
                }
        }
        false
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(true, contains_nearby_duplicate(vec![1,2,3,1], 3));
    }
}
