/* 
        array of length n sorted in ascending order is rotated between 1 and n times
        For example, the array nums = [0,1,2,4,5,6,7] might become:
                [4,5,6,7,0,1,2] if it was rotated 4 times. (*Rotated right)
                [0,1,2,4,5,6,7] if it was rotated 7 times. (*Rotated right)
        Notice that rotating an array  [a[0], a[1], a[2], ..., a[n-1]] to the right 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
        Given the sorted rotated array nums of unique elements, return the minimum element of this array.
                ^^^^ where n == arr.len(), min element is at idx = 0                
                ^^^^ where n < arr.len(), min element is at idx = (0 + n rotations), 
        Algorithm must run in O(log n) time (sub linear => divide and conquer strategy)
        Constraints:
                n == nums.length
                1 <= n <= 5000
                -5000 <= nums[i] <= 5000
                All the integers of nums are unique.
                nums is sorted and rotated between 1 and n times.
        Solution: max is idx 0 when array is completely un-rotated
                sliding window

*/
/* find partition index, binary search each half until we find answer */
pub fn find_min(nums: Vec<i32>) -> i32 {
        let split = nums.partition_point(|&x| x >= nums[0]);
        if split == nums.len() {
                nums[0]
        } else {
                nums[split]
        }
        // if target > nums[0] { // search 0..split
                
        // } else { // search split..nums.len()

        // }
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(0, find_min(vec![0,1,2,3,4]));
                assert_eq!(1, find_min(vec![3,4,5,1,2]));
                assert_eq!(0, find_min(vec![4,5,6,7,0,1,2]));
        }
}
