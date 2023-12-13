fn max_product(nums: Vec<i32>) -> i32 {
        let mut gold = nums[0];
        let mut silver = 0;
        
        for i in 1..nums.len() {
                if gold < nums[i] {
                        silver = gold;
                        gold = nums[i];
                } else if silver < nums[i] {
                        silver = nums[i]
                }
        }
        (gold - 1) * (silver - 1)
}
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(max_product(vec![2,5,1,4]), 12);
        }
}
