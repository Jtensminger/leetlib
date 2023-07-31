
// binary search on a possibly rotated array
// Find Pivot Index + Binary Search

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let split = nums.partition_point(|&x| x >= nums[0]);
        if target < nums[0] {
                if let Ok(i) = nums[split..].binary_search(&target) {
                        (i + split) as i32
                } else {
                        -1
                }
        } else {
                if let Ok(i) = nums[..split].binary_search(&target) {
                        i as i32
                } else {
                        -1
                }
        }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(4, search(vec![4,5,6,7,0,1,2], 0));
        assert_eq!(-1, search(vec![4,5,6,7,0,1,2], 3));
        assert_eq!(-1, search(vec![1], 0));
    }
}
