/* #TAGS[Binary Search, Array] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/binary-search/] */

use std::cmp::Ordering::{Greater,Equal,Less};
fn search(nums: Vec<i32>, target: i32) -> i32 {        
        if nums.len() == 0 { return -1 }
        let (mut start, mut end) = (0, nums.len());
        while start < end {
                let mid = start + (end - start) / 2;
                match &target.cmp(&nums[mid]) {
                        Equal => return mid as i32,
                        Less => start = mid,
                        Greater => end = mid + 1
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