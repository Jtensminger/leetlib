
use std::collections::HashSet;
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<_> = nums1.iter().cloned().collect();
        let set2: HashSet<_> = nums2.iter().cloned().collect();
        set1.intersection(&set2).cloned().collect()
}

#[cfg(test)]
pub mod tests {
        use super::*;
        /* Given two integer arrays nums1 and nums2, return an array of their intersection.
        Each element in the result must be unique and you may return the result in any order. */
        #[test]
        fn ext1() {
                let (n1, n2, target) = (vec![1,2,2,1], vec![2,2], vec![2]);
                assert_eq!(target, intersection(n1, n2))
        }

        #[test]
        fn ext2() {
                let (n1, n2, t1, t2) = (vec![4,9,5], vec![9,4,9,8,4], vec![9,4], vec![4,9]);
                let output = intersection(n1, n2);
                assert!(output == t1 || output == t2, "failed")
        }

        #[test]
        fn ext3() {
                let (n1, n2, t1, t2) = (vec![4,7,9,7,6,7], vec![5,0,0,6,1,6,2,2,4], vec![6,4], vec![4,6]);
                let output = intersection(n1, n2);
                assert!(output == t1 || output == t2, "failed")
        }
}
