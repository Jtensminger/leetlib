pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = Vec::with_capacity(2*n);
        for i in 0..n {
            res.push(nums[i]);
            res.push(nums[i+n]);
        }
        res
}
mod tests {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(shuffle(vec![2,5,1,3,4,7], 3), vec![2,3,5,4,1,7])
    }
}
