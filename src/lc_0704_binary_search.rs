use std::cmp::Ordering::{Greater,Equal,Less};
// if nums.contains(target) == return index
// else return -1

// Solved using in-built Binary Search method
// pub fn search(nums: Vec<i32>, target: i32) -> i32 {        
//         return match nums.binary_search(&target) {
//                 Ok(i) => i as i32,
//                 _ => -1
//         }
// }

pub fn search(nums: Vec<i32>, target: i32) -> i32 {        
        if nums.len() == 0 { return -1 }
        let mut l = 0;
        let mut h = nums.len();

        while l < h {
                let m = l + (h - l) / 2;
                match &nums[m].cmp(&target) {
                        Equal => return m as i32,
                        Less => l = m + 1,
                        Greater => h = m,
                }
        }
        -1
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4)
        }

        #[test]
        fn ext2() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1)
        }
}