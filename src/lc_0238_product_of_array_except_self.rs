        /*  1   2   3   4  */
        /*  24  12  8   6  */
        /*  0   1   2   3  */
/* output[i] = nums[..i] * nums[i+1..] */

// pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         (0..nums.len())
//                 .map(|i| nums[..i].iter().product::<i32>() * nums[i+1..].iter().product::<i32>())
//                 .collect::<Vec<i32>>()
// }

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![1; nums.len()];
        (1..nums.len()) /* indices for accessing prev elements */
                .for_each(|i| output[i] = nums[i - 1] * output[i - 1]);
        let mut suffix = 1;
        for (i, n) in output.iter_mut().enumerate().rev(){
                *n = *n * suffix;
                suffix *= nums[i];
        }
        output
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let nums = vec![1, 2, 3, 4];
                assert_eq!(vec![24, 12, 8, 6], product_except_self(nums));
        }

        #[test]
        fn ext2() {
                let nums = vec![-1, 1, 0, -3, 3];
                assert_eq!(vec![0, 0, 9, 0, 0], product_except_self(nums));
        }
}
/* 
Problem:
        Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
        The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
        You must write an algorithm that runs in O(n) time and without using the division operation.
Constraints:
        2 <= nums.length <= 10^5
        -30 <= nums[i] <= 30
        The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer
Follow up:
        Can you solve the problem in O(1) extra space complexity? 
        (The output array does not count as extra space for space complexity analysis.)
 */