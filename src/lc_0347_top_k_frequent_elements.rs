/*
Problem: 
	Given an integer array nums and an integer k, return the k most frequent elements.
	You may return the answer in any order.
Constraints:
	1 <= nums.length <= 10^5
	-10^4 <= nums[i] <= 10^4
	k is in the range [1, the number of unique elements in the array].
	It is guaranteed that the answer is unique.

Follow up:
	Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
OLD NOTES: 
	Algo: O(n * mlogm)
		create hashmap of nums and counts.
		sort hashmap values by count
		return kth most requent values form sorted list
	Algo: O(n * k)
		create hashmap to hold unique nums and counts
		store most k most frequent numbers in k-capacity array
		for each num:
			store or update hashmap += 1
			compare:
				for i in 0..kth most frequent numbers
					if num > kth_most_frequent_number:
						kth_most_frequent_number[i] = num
		return k_most_frequent_numbers
*/

use std::collections::HashMap;
use std::cmp::Ordering;
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
	let k = k as usize;
	// Map each unique num to it's frequency in the list: map[num] => num_frequency
	let mut map: HashMap<i32, i32> = HashMap::with_capacity(k);
	nums.iter().for_each(|num| *map.entry(*num).or_default() += 1);

	// Create unordered list of (num, frequency) pairs: num_frequency_pairs[idx] => (num, num_frequency)
	let mut num_frequency_pairs: Vec<(i32, i32)> = map.into_iter().collect();

	// Reduce the list of num_frequency_pairs to just the top k items based on frequency
	let top_k_pairs = match k == num_frequency_pairs.len() {
                true  => &num_frequency_pairs,
                false => quick_select(&mut num_frequency_pairs, k as i32)
        };
	
	//  return final list of nums after reducing list of top_k_pairs to just the nums
	top_k_pairs.into_iter()
		.map(|&(n, _)| n)
		.collect()
}

/*
	Quick Select
	Algorithm Overview: 
		Find the kth smallest item in an unordered list, then partition around it.
		(i.e reorder items smaller than the kth item to the left and larger items to the right.)
	Input/Output: quick_select([3,1,2,2,1,1], 2) -> [1,2]
	Logic:
		1) 	Cut slice in-half down the middle to create a left and right
		2) 	Select pivot value (randomly or deterministically) that will determine which side values move to.
		4)	For each elem in slice, compare elem < pivot
		5) 		If index equals k: we have found the k-th smallest element and we return
				If index of the partitioned elem is greater_than k: recur left
				If index is less_than k: recur right
		Output: TBD
*/ 

pub fn quick_select(slice: &mut [(i32, i32)], k: i32) -> &[(i32, i32)] {
	// Select a pivot index
	let (mut pivot_idx, mut i_idx, mut j_idx) = (0, 1, 1);
	// Iterate over the slice (skipping the pivot), comparing each elem against the Pivot
	for current_idx in 1..slice.len() {	
		// Extracting current & pivot from our indexes
		let (current, pivot) = (slice[current_idx].1, slice[pivot_idx].1);
		// TBD
		if current >= pivot {
			slice.swap(current_idx, j_idx);
			j_idx += 1;
		} else {
			slice.swap(current_idx, i_idx); 
			i_idx += 1; 
			j_idx += 1;
		}
	}
	// TBD
	slice.swap(pivot_idx, i_idx - 1);
	pivot_idx = i_idx - 1;
	let larger_items = (j_idx - pivot_idx) as i32;
	match larger_items.cmp(&k) {
		Ordering::Less => quick_select(&mut slice[0..j_idx], k),
		Ordering::Greater => quick_select(&mut slice[pivot_idx + 1..j_idx], k),
		Ordering::Equal => &slice[pivot_idx..j_idx],
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ext1() {
		assert_eq!(vec![2,1], top_k_frequent(vec![1,1,1,2,2,3], 2));
	}

	#[test]
	fn ext2() {
		assert_eq!(vec![1], top_k_frequent(vec![1], 1));
	}
}



// pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
// 	let k = k as usize;
// 	let mut k_freq_num: Vec<i32> = vec![1; k];
// 	let mut k_freq_cnt: Vec<i32> = vec![1; k];
// 	let mut map: HashMap<i32, i32> = HashMap::with_capacity(k);
// 	for n in nums {
// 		// store or update hashmap += 1
// 		let cnt = map.entry(n)
// 			.and_modify(|cnt| *cnt+=1)
// 			.or_insert(1);
		
// 		// determine if it belongs in the list
// 		if *cnt >= k_freq_cnt[0] {
// 			// where it belongs in the list
// 			let idx = match k_freq_cnt.binary_search(cnt) {
// 				Ok(idx) => if idx != 0 { idx - 1 } else { idx },
// 				Err(idx) => if idx != 0 { idx - 1 } else { idx },
// 			};
// 			k_freq_cnt[idx] = *cnt;
// 			k_freq_num[idx] = n;
// 		}
// 	}
// 	k_freq_num
// }
// // let (mut k_freq_num, mut k_freq_cnt): (Vec<i32>, Vec<i32>) = map.into_iter().unzip();