pub fn is_valid(s: String) -> bool {
	if s.len() % 2 != 0 || s.len() == 0 {
		return false
	}

	let mut peekable = s.chars().peekable();
	let mut stack: Vec<char> = vec![];

	while peekable.peek() != None {
		let c = peekable.next();
		match c {
			Some(n @ '{') => if peekable.peek() == None { return false } else { stack.push(n); },
			Some(n @ '(') => if peekable.peek() == None { return false } else { stack.push(n); },
			Some(n @ '[') => if peekable.peek() == None { return false } else { stack.push(n); },
			Some('}') => if stack.last() != Some(&'{') { return false } else { stack.pop(); },
			Some(')') => if stack.last() != Some(&'(') { return false } else { stack.pop(); },
			Some(']') => if stack.last() != Some(&'[') { return false } else { stack.pop(); },
			_ => return false,
		}
	}
	stack.len() == 0
}

		// match c {
		// 	Some('{') => if peekable.find(|&x| x == '}') == None {return false},
		// 	Some('(') => if peekable.find(|&x| x == ')') == None {return false},
		// 	Some('[') => if peekable.find(|&x| x == ']') == None {return false},
		// 	_ => return false,
		// }

		// match c {
		// 	'{' => if (n != '}') {return false},
		// 	'(' => if (n != ')') {return false},
		// 	'[' => if (n != ']') {return false},
		// 	_ => return false,
		// }

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
 