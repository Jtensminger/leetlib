use std::collections::HashMap;
        // map of # frequency
        // iterate updating frequency && compare against max && update max
        // return max
        // let majority: (usize, i32) = (0, 0);

pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.into_iter().fold((0, 0),  |(maj, maj_count), e| {
                let curr_count = map.entry(e).and_modify(|v| { *v += 1; }).or_insert(1);
                
                if  *curr_count > maj_count { (e, *curr_count) } 
                else { (maj, maj_count) }
        }).0
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(3, majority_element(vec![3,2,3]))
        }

        #[test]
        fn ext2() {
                assert_eq!(2, majority_element(vec![2,2,1,1,1,2,2]))
        }
}