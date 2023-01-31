use std::collections::HashMap;

// BRUTE FORCE
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	
	let mut hm: HashMap<i32, i32> = HashMap::new();

	// iterate through numbers
	for (index, num) in nums.into_iter().enumerate() {
		let complement = target - num;

		if let Some(comp_index) = hm.get(&complement) {
			return vec![*comp_index, index as i32]
		}

		hm.entry(num).or_insert(index as i32);

	}
	
	vec![]
}





#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ext1() {
		assert_eq!(two_sum(vec![11, 2, 15, 7], 9), vec![1, 3])
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
 