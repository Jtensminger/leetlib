pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Vec::from([i as i32, j as i32]);
            }
        }
    }
    Vec::<i32>::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(two_sum(vec![11, 2, 15, 7], 9), vec![0, 1])
    }

    #[test]
    fn ext2() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }

    #[test]
    fn ext3() {
        assert_eq!(two_sum(vec![3, 7, 0, 1], 7), vec![1, 2])
    }

    #[test]
    fn ext4() {
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2])
    }
}
 