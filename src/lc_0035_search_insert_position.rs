use std::cmp::Ordering::{Greater, Equal, Less};

/* 
Problem: 
        Given a sorted array of distinct integers and a target value, return the index if the target is found.
        If not, return the index where it would be if it were inserted in order.
        You must write an algorithm with O(log n) runtime complexity.
Constraints:
        1 <= nums.length <= 10^4
        -10^4 <= nums[i] <= 10^4
        nums contains distinct values sorted in ascending order.
        -10^4 <= target <= 10^4
*/
        /* ascending order: 0,1,2... */
        /* return idx of target || idx to insert target */
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        recurse(&nums[..], target)
}
pub fn recurse(nums: &[i32], target: i32) -> i32 {
        let mid = nums.len() / 2;
        /* base_case: target isn't in nums */
        if nums.len() == 1 {
                return if target <= nums[mid] {
                     mid as i32
                } else {
                    (mid as i32) + 1
                }
        } else if nums.len() == 0 {
                return mid as i32
        }
        /* walk the side of the array ~possibly~ containing target  */
        match target.cmp(&nums[mid]) {
                Equal => mid as i32,
                Less => recurse(&nums[..mid], target),
                Greater => {
                        let right_idx = recurse(&nums[mid+1..], target);
                        (mid as i32) + 1 + right_idx /* offset to the result of the recursive call */
                }
        }
}


#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let nums = vec![1,3,5,6];
                let target = 5;
                let expected = 2;
                assert_eq!(expected, search_insert(nums, target));
        }

        #[test]
        fn ext2() {
                let nums = vec![1,3,5,6];
                let target = 2;
                let expected = 1;
                assert_eq!(expected, search_insert(nums, target));
        }

        #[test]
        fn ext3() {
                let nums = vec![1,3,5,6];
                let target = 7;
                let expected = 4;
                assert_eq!(expected, search_insert(nums, target));
        }
}
