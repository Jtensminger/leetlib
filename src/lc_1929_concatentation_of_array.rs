pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        vec![nums.clone(), nums].concat()
}

mod test {
    use super::*;

    #[test]
    fn ext1() {
        let nums = vec![1,2,1];
        let ans = get_concatenation(nums);
        assert_eq!(vec![1,2,1,1,2,1], ans);
    }
}
