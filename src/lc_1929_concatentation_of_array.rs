pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        vec![nums.clone(), nums].concat()
}

/* Alternative way

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext1() {
        let nums = vec![1,2,1];
        let ans = get_concatenation(nums);
        assert_eq!(vec![1,2,1,1,2,1], ans);
    }
}
