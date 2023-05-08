/* 
Problem:
    Given an integer array nums sorted in non-decreasing order, 
    return an array of the squares of each number sorted in non-decreasing order.

Constraints:
        1 <= nums.length <= 104
        -104 <= nums[i] <= 104
        nums is sorted in non-decreasing order.

Follow up:
        Could you find an O(n) solution using a different approach?

Naive Algo: O( n^2 log(n) )
        square numbers  O(n)
        sort            O(n log(n))

Optimized Algo: reduce to O(n^2), O(n log n), or O(n)
        min = nums.first()
        max = nums.last()
        if nums[i]^2 >= nums[i+1]^2: swap the


*/


pub fn naive_sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut output = nums
                .iter_mut()
                .map(|&mut k| k * k)
                .collect::<Vec<i32>>();
                
        output.sort();
        output
}

/*
Algo: Optimized Sorted_Squares
Complexity: O(N)
------------------------------------------------------------
input        [-2,       1,        3]       
------------------------------------------------------------
initialize   min         max      (Unchecked)
             [-2,         1,        3]
------------------------------------------------------------
loop max<len min         max      (Unchecked)
             [-2,         1,        3]
------------------------------------------------------------
             min         max      (Unchecked)
n^2          [-2,         1,        3]
------------------------------------------------------------
             min         max      (Unchecked)
cmp(4<1)     [4,         1,        3]
------------------------------------------------------------
             min         max      (Unchecked)
swap         [1,         4,        3]
------------------------------------------------------------
             (processed) min      max
shift        [1,         4,        3]
------------------------------------------------------------
loop max<len (processed) min      max
             [1,         4,        3]
*/  

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0;nums.len()];
        let mut left: usize = 0; 
        let mut right: usize = nums.len() - 1;
        
        for idx in (0..nums.len()).rev() {
                if nums[left].abs() > nums[right].abs() {        
                        res[idx] = nums[left] * nums[left];
                        left += 1;
                } else {
                        res[idx] = nums[right] * nums[right];
                        right -= 1;
                }
        }
        res
}

#[cfg(test)]
mod test {
        use super::*;
        #[test]
        fn ext1_optimized() {
                // 0 <= n <= 104
                let nums   = vec![-3, -2];
                let target = vec![4, 9];
                assert_eq!(target, sorted_squares(nums));

                // -104 <= n <= 104
                let nums   = vec![-2, 0, 1];
                let target = vec![0, 1, 4];
                assert_eq!(target, sorted_squares(nums));
        }
        
        #[test]
        fn ext2_naive() {
                // 0 <= n <= 104
                let nums   = vec![0, 2];
                let target = vec![0, 4];
                assert_eq!(target, naive_sorted_squares(nums));

                // -104 <= n <= 104
                let nums   = vec![-2, 0, 1];
                let target = vec![0, 1, 4];
                assert_eq!(target, naive_sorted_squares(nums));
        }
}
