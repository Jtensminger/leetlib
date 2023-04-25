/* 
    Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
    Return the array in the form [x1,y1,x2,y2,...,xn,yn]
*/
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; nums.len()];

        for i in 0..n {
            result[2 * i] = nums[i];
            result[2 * i + 1] = nums[n + i];
        }

        result
}


mod tests {
    use super::*;

    #[test]
    fn ext1() {
        let (n, nums) = (3, vec![2,5,1,3,4,7]);
        assert_eq!(vec![2,3,5,4,1,7], shuffle(nums, n));
    }
}
