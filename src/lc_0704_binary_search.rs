/* #TAGS[Binary Search, Array] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/binary-search/] */

use std::cmp::Ordering::{Greater,Equal,Less};
pub fn search(nums: Vec<i32>, target: i32) -> i32 {        
        if nums.len() == 0 { return -1 }
        let (mut left, mut right) = (0, nums.len());
        while left < right {
                let mid = left + (right - left) / 2;
                match &target.cmp(&nums[mid]) {
                        Equal => return mid as i32,
                        Less => right = mid,
                        Greater => left = mid + 1
                }
        }
        -1
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4);
        }

        #[test]
        fn ext2() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1)
        }
}