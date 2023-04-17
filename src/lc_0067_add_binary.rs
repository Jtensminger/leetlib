/* 
	Prompt: Given two binary strings a and b, return their sum as a binary string.
	
	Constraints: 
		1 <= a.length, b.length <= 104 
		a and b consist only of '0' or '1' characters.
		Each string does not contain leading zeros except for the zero itself.
*/

use std::collections::VecDeque;

pub fn add_binary(a: String, b: String) -> String {
        if a.is_empty() { return b; }
        if b.is_empty() { return a; }

	let mut a_chars = a.chars().rev();
	let mut b_chars = b.chars().rev();

	let mut sum = VecDeque::new();
	let mut carry = false;

        let mut next_pair = || {
		match (a_chars.next(), b_chars.next()) {
		    (None, None) => None,
		    (a, b) => Some((to_value(a), to_value(b))),
		}
	};
    
	while let Some((l, r)) = next_pair() {
		match (carry, l, r) {
			(false, false, false) => sum.push_front('0'),
			(false, true, false) | (false, false, true) | (true, false, false) => {
				sum.push_front('1');
				carry = false;
			},
			(false, true, true) | (true, false, true) | (true, true, false) => { 
				sum.push_front('0');
				carry = true;
			}
			(true, true, true) => {
				sum.push_front('1');
				carry = true;
			}
		}
        }

	if carry {
		sum.push_front('1');
	}

	sum.iter().collect::<String>()
}

fn to_value(c: Option<char>) -> bool {
	match c {
	    None => false,
	    Some('0') => false,
	    Some('1') => true,
	    Some(x) => panic!{"Unexpected char passed to value_for: {}", x},
	}
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                // Add your test code here
                let a = String::from("11");
                let b = String::from("1");
                let output = String::from("100");

                assert_eq!(output, add_binary(a, b))
        }

        #[test]
        fn ext2() {
                // Add your test code here
                let a = String::from("1010");
                let b = String::from("1011");
                let output = String::from("10101");

                assert_eq!(output, add_binary(a, b))
        }

	#[test]
        fn ext3() {
                // Add your test code here
                let a = String::from("0");
                let b = String::from("0");
                let output = String::from("0");

                assert_eq!(output, add_binary(a, b))
        }

	#[test]
        fn ext4() {
                // Add your test code here
                let a = String::from("1111");
                let b = String::from("1111");
                let output = String::from("11110");

                assert_eq!(output, add_binary(a, b))
        }
}

