use std::collections::BTreeSet;
fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let index_diff = index_diff as usize;
        // Acts as a window for items within a `k` wide range.
        let mut window = BTreeSet::new();
        for end in 0..nums.len() {
                // Get the incoming number and calculate the range of other numbers that would produce a value <= value_diff.
                let (upper, lower) = (nums[end] + value_diff, nums[end] - value_diff);
                // Check if the window holds any values within range.
                if window.range(lower..=upper).next() != None {
                        return true;
                }
                // Add incoming number to window.
                window.insert(nums[end]);
                // If window is full, remove the number going out of scope.
                if end >= index_diff {
                        window.remove(&nums[end - index_diff]);
                }
        }
        false
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                /*      
                        We can choose (i, j) = (0, 3). We satisfy the three conditions:
                        i != j --> 0 != 3
                        abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
                        abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
                */
                assert_eq!(true, contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0));
        }
        #[test]
        fn ext2() {
                /* After trying all the possible pairs (i, j), we cannot satisfy the three conditions, so we return false. */
                assert_eq!(false, contains_nearby_almost_duplicate(vec![1,5,9,1,5,9], 2, 3));
        }

        #[test]
        fn ext3() {
                // value_diff = 0 => duplicate number
                // index_diff = 3 => with-in 3 spots
                assert_eq!(true, contains_nearby_almost_duplicate(vec![1,2,2,3,4,5], 3, 0));
        }
        
}
