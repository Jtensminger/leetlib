use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
	let mut map = HashMap::new();
	for i in nums {
		if map.contains_key(&i) {
			return true
		}
		
		map.insert(i, true);
	}

	false
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let nums_true = vec![1,2,3,1];
                assert_eq!(true, contains_duplicate(nums_true));

                let nums_false = vec![1,2,3,4];
                assert_eq!(false, contains_duplicate(nums_false));
        }
}
