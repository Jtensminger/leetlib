pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums
            .iter()
            .map(|&x| nums[x as usize])
            .collect()
}

mod test {
    use super::*;

    #[test]
    fn ext1() {
        let nums = vec![0,2,1,5,3,4];
        let ans = build_array(nums);
        assert_eq!(vec![0,1,2,4,5,3], ans);
    }
}
