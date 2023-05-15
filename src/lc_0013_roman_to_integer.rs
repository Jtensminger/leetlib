
/*
Problem: Given a roman numeral, convert it to an integer.
Constraints:
        1 <= s.length <= 15
        s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
        It is guaranteed that s is a valid roman numeral in the range [1, 3999].

I             1
V             5
X             10
L             50
C             100
D            500
M             1000
Subtraction Rules:
        I can be placed before V (5) and X (10) to make 4 and 9.
        X can be placed before L (50) and C (100) to make 40 and 90. 
        C can be placed before D (500) and M (1000) to make 400 and 900.
 */
/*
Algo:
        let mut acc: i32 = 0; 
        Walk the String in pairs:
                (last, current) pattern match to get value from letter
                ('I', 'V') | ('I', 'X') => subtract I
		('X' 'L') | ('X' 'C') => subtract X
                ('C' 'D') | ('C' 'M') => subtract C
                (_, _) => Add
                acc += value;
        return acc
*/

pub fn roman_to_int(s: String) -> i32 {
	let mut acc: i32 = 0; 
	let mut s = s.chars().peekable();
	while let Some(c) = s.next() {
		let mut value= match c {
			'I' => 1,
			'V' => 5,
			'X' => 10,
			'L' => 50, 
			'C' => 100,
			'D' => 500,
			'M' => 1000,
			_ => 0,
		};
		if let Some(peeked) = s.peek() {
			value = match (c, peeked) {
				('I', 'V') | ('I', 'X') => value * -1,
				('X','L') | ('X','C') => value * -1,
				('C','D') | ('C','M') => value * -1,
				_ => value,
			};
		}
		acc += value;
	}
	acc
}

pub mod tests {
	use super::*;

	#[test]
	fn only_add() {
		assert_eq!(3, roman_to_int(String::from("III")));
	}
	#[test]
	fn subtract() {
		assert_eq!(4, roman_to_int(String::from("IV")));
	}
}
