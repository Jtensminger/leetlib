pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len() - 1;
        let mut sum: i32 = numbers[left] + numbers[right];
        while sum != target {
                if sum > target {
                        right -= 1;
                } else {
                        left += 1;
                }
                sum = numbers[left] + numbers[right];
        }
        vec![left as i32 + 1, right as i32 + 1]
}

/* 
        SOLVED WHEN: SUM(nums[l], nums[r]) == target
        target_i = idx of the target if it existed in the array
        (1..1000)   => l & r < target_i                   
        (-1000..-1) => l & r > target_i
*/
#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                /* The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2]. */
                let   actual = two_sum(vec![2,7,11,15], 9);
                let expected = vec![1,2];
                assert_eq!(expected, actual);

                /* The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3]. */
                let   actual = two_sum(vec![2,3,4], 6);
                let expected = vec![1,3];
                assert_eq!(expected, actual);

                /* The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2]. */
                let   actual = two_sum(vec![-1, 0], -1);
                let expected = vec![1,2];
                assert_eq!(expected, actual);
        }
}
