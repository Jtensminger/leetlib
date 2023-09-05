/* 
A peak element is an element that is strictly greater than its neighbors
Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks. 
An element is always considered to be strictly greater than a neighbor that is outside the array.
You must write an algorithm that runs in O(log n) time.
*/
fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
                return 0
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
                let mid = left + (right - left) / 2;
                if mid > 0 && nums[mid] < nums[mid - 1] { // check left neighbor
                        right = mid - 1;
                } else if mid < nums.len() - 1 && nums[mid] < nums[mid + 1] { // check right neighbor
                        left = mid + 1;
                } else {
                        return mid as i32;
                }
        }
        -1
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(2, find_peak_element(vec![1,2,3,1]));
                assert_eq!(5, find_peak_element(vec![1,2,1,3,5,6,4]));
        }
}
