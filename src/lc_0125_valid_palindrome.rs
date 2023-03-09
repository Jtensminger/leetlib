/* #TAGS[Two Pointers, String, Double-ended queue] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/valid-palindrome/] */

/*
A phrase is a palindrome if...
        after converting all uppercase letters into lowercase letters
        it reads the same forward and backward. 
        and removing all non-alphanumeric characters, (Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

Constraints:
        1 <= s.length <= 2 * 105
        s consists only of printable ASCII characters.

*/


/*
        ""
        len() = 0, even
        invariant: input == 0 -> return true

        "amanaplanacanalpanama"
        len() = 21, odd
        middle_elem[len / 2 = 10] = c 
        invariant: 
                >> string is forward--backward symmetrical

                >> if split in half into two strings, discarding the middle,
                        left == right.reverse()

                >> elem[i] == elem[(string_len - 1) - i]

        "raceacar"
        len() = 8, even 
        invariant:
        
 */

pub fn is_palindrome(s: String) -> bool {
        
        let mut filtered_input = s.chars()
                .filter_map( |c| 
                        c.is_ascii_alphanumeric()
                        .then(|| c.to_ascii_lowercase()));

        while let (Some(l), Some(r)) = (filtered_input.next(), filtered_input.next_back()) {
                if r != l { return false }
        }

        return true;
}



#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                //let mut filtered_input = String::from("amanaplanacanalpanama");
                assert!(is_palindrome(String::from("A man, a plan, a canal: Panama")), "contains a logic bug");
        }

        #[test]
        fn ext2() {
                //let mut filtered_input = String::from("raceacar");
                assert_eq!(false, is_palindrome(String::from("race a car")), "contains a logic bug");
        }

        #[test]
        fn ext3() {
                assert!(is_palindrome(String::from("")), "contains a logic bug");
        }
}