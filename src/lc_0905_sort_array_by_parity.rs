/*
Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
Return any array that satisfies this condition.
*/

/* Alternative Approaches
create 2 vecs, walk nums, push even to vec1, push odd to vec2, concat them at the end
create hashmap, walk nums, count freq of each number, reconstruct array using the keys and counts
*/
pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut odd:  Vec<i32> = Vec::with_capacity(nums.len() / 2);
        let mut even: Vec<i32> = Vec::with_capacity(nums.len() / 2);

        for idx in 0..nums.len() {
                let num = nums[idx];
                if num % 2 == 0 {
                        even.push(num);
                } else {
                        odd.push(num);
                }
        }
        even.append(&mut odd);
        even
}


#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
            assert_eq!(sort_array_by_parity(vec![3,1,2,4]), vec![2,4,3,1]);
            assert_eq!(sort_array_by_parity(vec![0]), vec![0]);
        }
}
