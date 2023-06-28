/* Problem: Given a string s, find the length of the longest substring without repeating characters. */
/* Initial Thoughts 
        walk string, check if each char is in the bag already:
                if not in bag, put it in the bag, increment lenght counter, move to next char
                if it's in the bag, store max(length, max_length), reset length counter to 1, empty the bag, store current char in bag, move to next char 
*/



/* naive approach */
use std::collections::HashSet;
use std::collections::VecDeque;
pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 1 || s.len() == 0 { return s.len() as i32 }
        let mut deq = VecDeque::new();
        let mut bag = HashSet::with_capacity(s.len());
        let mut max_len = 0;
        for letter in s.chars() {
                while bag.contains(&letter) {
                        bag.remove(&deq.pop_front().unwrap());
                }
                deq.push_back(letter);
                bag.insert(letter);
                max_len = max_len.max(deq.len());
        }
        max_len as i32
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
                assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
                assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
                assert_eq!(2, length_of_longest_substring("aab".to_string()));
                assert_eq!(3, length_of_longest_substring("dvdf".to_string()));
                assert_eq!(5, length_of_longest_substring("anviaj".to_string()));
        }
}
