pub fn is_palindrome(x: i32) -> bool {
	let y: String = x.to_string().chars().rev().collect();
	y == x.to_string()
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(true, is_palindrome(121));
		assert_eq!(false, is_palindrome(-121));
        }
}
