
/* 
You are given the string croakOfFrogs, which represents a combination of the string "croak" from different frogs,
that is, multiple frogs can croak at the same time, so multiple "croak" are mixed.

Return the minimum number of different frogs to finish all the croaks in the given string.

A valid "croak" means a frog is printing five letters 'c', 'r', 'o', 'a', and 'k' sequentially.
The frogs have to print all five letters to finish a croak. If the given string is not a combination of a valid "croak" return -1.

*/

// key insights:
// *= mapping "croak" to array indices prevents us from typing out all matching conditions. just arr[n-1] to see if the previous letter was set. if not, return -1
// *= count the frogs when they start croaking & remove one when they start. This tracks the current number of frogs needed for overlapping croaks
// *= storing the largest/max number of frogs every concurrently croaking allows us to find our answer as we iterate the string.

pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
	let mut cnt = [0; 5]; // croak
	let mut frogs = 0; 
	let mut max_frogs = 0;

	for ch in croak_of_frogs.chars() {
		let n = "croak".find(ch).unwrap();
		cnt[n]+=1;

		match n {
			0 => {
				frogs +=1;
				max_frogs = max_frogs.max(frogs);
			},
			4 => frogs -= 1,
			_ => {
				if cnt[n - 1] == 0 {
					return -1;
				}
				cnt[n - 1] -= 1;
			},
		}
	}
	if frogs != 0 {
		return -1;
	}

	max_frogs
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ext1() {
		let croak_of_frogs = String::from("croakcroak");
		let result = min_number_of_frogs(croak_of_frogs);
		assert_eq!(1, result);
	}

	#[test]
	fn ext2() {
		let croak_of_frogs = String::from("crcoakroak");
		let result = min_number_of_frogs(croak_of_frogs);
		assert_eq!(2, result);
	}

	#[test]
	fn ext3() {
		let croak_of_frogs = String::from("croakcrook");
		let result = min_number_of_frogs(croak_of_frogs);
		assert_eq!(-1, result);
	}

	#[test]
	fn ext4() {
		let croak_of_frogs = String::from("croakcroa");
		let result = min_number_of_frogs(croak_of_frogs);
		assert_eq!(-1, result);
	}
}
