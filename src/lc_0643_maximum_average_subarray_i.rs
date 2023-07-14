/* 
You are given an integer array nums consisting of n elements, and an integer k.
Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value.
Any answer with a calculation error less than 10^(-5) will be accepted.
"contiguous subarray whose length is equal to k" -> constraint
"maximum average value" -> question
*/
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 { 
        let k = k as usize;
        let mut running_sum: i32 = nums[0..k].iter().sum();
        let mut max_sum = running_sum;
        for end in k..nums.len() {
                running_sum += nums[end] - nums[end - k]; // constraint is k window size
                max_sum = max_sum.max(running_sum);
        }
        max_sum as f64 / k as f64 // question
}


#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(12.75000, find_max_average(vec![1,12,-5,-6,50,3], 4));
        }
}
