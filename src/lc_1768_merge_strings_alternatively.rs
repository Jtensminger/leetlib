// Dynamic Programming Problem
/* Problem:
	You are given two strings word1 and word2.
	Merge the strings by adding letters in alternating order, starting with word1.
	If a string is longer than the other, append the additional letters onto the end of the merged string.
	Return the merged string.
*/

// Insights
// *= # of iterations = longest word
// *= by looping the longest iteration, I don't have create code for handing the remaining characters of the longer string
// *= by using iterators & pattern matching instead of accessing a list via index, I never have to worry about index out of bounds errors
// *= this follows the one-pointers structure but uses pattern matching instead of pointers

// Complexity Analysis: m = word1.len(), n = word2.len()
// Time complexity: O(MAX(m,n))
// Space complexity: O(1)
pub fn merge_alternately(word1: String, word2: String) -> String {
	let mut w1 = word1.chars();
	let mut w2 = word2.chars();
	let mut merged = String::new();
	
	for _ in 0..word1.len().max(word2.len()) {
		if let Some(ch) = w1.next() {
			merged.push(ch)
		}

		if let Some(ch) = w2.next() {
			merged.push(ch)
		}
	}	
	merged
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ext1() {
		let (w1, w2, answer) = (String::from("abc"), String::from("pqr"), String::from("apbqcr"));
		assert_eq!(answer, merge_alternately(w1, w2));
	}

	#[test]
	fn ext2() {
		let (w1, w2, answer) = (String::from("ab"), String::from("pqrs"), String::from("apbqrs"));
		assert_eq!(answer, merge_alternately(w1, w2));
	}

	#[test]
	fn ext3() {
		let (w1, w2, answer) = (String::from("abcd"), String::from("pq"), String::from("apbqcd"));
		assert_eq!(answer, merge_alternately(w1, w2));
	}
}
