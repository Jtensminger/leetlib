
/* 
        create a list of triplets [i, j, k] that satisfy these rules:
                no duplicates in triplets => i != j && i != k && j != k
                sum to zero => i + j + k == 0
*/
use std::collections::HashSet;
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /* find all possible triplets (e.g index combinations) & store the valid ones */
        let mut valid_triplets: HashSet<Vec<i32>> = HashSet::with_capacity(nums.capacity());
        for i in 0..nums.len() {
                for j in 0..nums.len() {
                        for k in 0..nums.len() {
                                let duplicates = i == j || i == k || j == k;
                                let non_zero_sum = nums[i] + nums[j] + nums[k] != 0;
                                if duplicates || non_zero_sum {
                                        continue
                                }
                                let mut valid_triplet = vec![nums[i], nums[j], nums[k]];
                                valid_triplet.sort();
                                /* save valid triplets */
                                if !valid_triplets.contains(&valid_triplet) {
                                        valid_triplets.insert(valid_triplet);
                                }
                        }
                }              
        }
        /* return valid triplets */
        valid_triplets.into_iter().collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
        use super::*;
        #[test]
        fn ext1() {
                let nums = vec![0,0,0];
                assert_eq!(vec![vec![0,0,0]], three_sum(nums));
        }
        #[test]
        fn ext2() {
                let nums = vec![-1,0,1,2,-1,-4];
                let mut solution_triplets = HashSet::from([vec![-1,-1,2], vec![-1,0,1]]);
                for triplet in three_sum(nums) {
                        assert!(solution_triplets.remove(&triplet));
                }
                assert!(solution_triplets.is_empty());
        }
}
