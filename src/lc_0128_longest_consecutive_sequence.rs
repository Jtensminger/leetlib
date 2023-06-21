/*
Problem: 
        Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
        You must write an algorithm that runs in O(n) time.
Constraints:
        0 <= nums.length <= 10^5
        -10^9 <= nums[i] <= 10^9
*/
use std::collections::HashSet;
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        /* create SET to check if neighbors existed */
        let set : HashSet<i32> = HashSet::from_iter(nums.into_iter());
        /* set max sequence to 0 since we haven't walked the set yet */
        let mut max_cnt = 0;
        for n in &set {
                /* if the preceding number is not in the set, it's the start of a sequence  */
                /* skip over numbers that aren't the start of a sequence */
                if !set.contains(&(n-1)) {
                        let mut next = n + 1;
                        let mut cnt = 1;
                        /* increase the sequence count while a consecutive next number exists */
                        while set.contains(&next) {
                                cnt += 1;
                                next += 1;
                        }
                        /* compare the sequence length to previous max length we stored */
                        max_cnt = max_cnt.max(cnt);
                }
        }
        /* return the length of longest sequence */
        max_cnt
}


#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let nums = vec![100,4,200,1,3,2];
                /*  The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4 */
                let expected = 4;
                assert_eq!(expected, longest_consecutive(nums));
        }

        #[test]
        fn ext2() {
                let nums = vec![0,3,7,2,5,8,4,6,0,1];
                let expected = 9;
                assert_eq!(expected, longest_consecutive(nums));
        }
}
