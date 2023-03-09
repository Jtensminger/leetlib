/* #TAGS[String, Stack] #DIFFICULTY[Easy] URL#[https://leetcode.com/problems/valid-parentheses/] */

pub fn is_valid(s: String) -> bool {

	let mut stack = Vec::new();
	for i in s.chars() {
		match i {
			'{' => stack.push('}'),
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'}' | ')' | ']' if Some(i) != stack.pop() => return false,
			_ => (),
		}	
	}
	stack.is_empty()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ext1() {
		assert_eq!(is_valid(String::from("()")), true)
	}

	#[test]
	fn ext2() {
		assert_eq!(is_valid(String::from("()[]{}")), true)
	}

	#[test]
	fn ext3() {
		assert_eq!(is_valid(String::from("(]")), false)
	}

	#[test]
	fn ext4() {
		assert_eq!(is_valid(String::from(")(")), false)
	}

	#[test]
	fn ext5() {
		assert_eq!(is_valid(String::from("{}(")), false)
	}
	#[test]
	fn ext6() {
		assert_eq!(is_valid(String::from("{[]}")), true)
		// {[
	}
	#[test]
	fn ex7() {
		assert_eq!(is_valid(String::from("{[(])}")), false)
	}

	#[test]
	fn ex8() {
		assert_eq!(is_valid(String::from("((")), false)
	}

	#[test]
	fn ex9() {
		assert_eq!(is_valid(String::from("))")), false)
	}
	#[test]
	fn ext10() {
		assert_eq!(is_valid(String::from("[[[]")), false)
	}
}
 