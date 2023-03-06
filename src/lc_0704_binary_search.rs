
// if nums.contains(target) == return index
// else return -1
pub fn search(nums: Vec<i32>, target: i32) -> i32 {        
        return match nums.binary_search(&target) {
                Ok(i) => i as i32,
                _ => -1
        }
}

#[cfg(test)]
mod test {
    use crate::search;

        
        
        #[test]
        fn ext1() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4)
        }

        #[test]
        fn ext2() {
                assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1)
        }
}