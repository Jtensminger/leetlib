/* Problem: Given a string s, find the length of the longest substring without repeating characters. */
/* Initial Thoughts 
        walk string, check if each char is in the bag already:
                if not in bag, put it in the bag, increment lenght counter, move to next char
                if it's in the bag, store max(length, max_length), reset length counter to 1, empty the bag, store current char in bag, move to next char 
*/




/* Sliding Window w/ VecQeque */
// use std::collections::VecDeque;
// pub fn length_of_longest_substring(s: String) -> i32 {
//         let mut deq = VecDeque::with_capacity(s.len());
//         let mut longest = 0;
//         for letter in s.chars() {
//                 while deq.contains(&letter) {
//                         deq.pop_front();
//                 }
//                 deq.push_back(letter);
//                 longest = longest.max(deq.len());
//         }
//         longest as i32
// }

/* Sliding Window w/ Set */
use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
        let (mut start, mut max_len) = (0, 0);
        let mut chars_seen = HashSet::with_capacity(s.len());
        for end in 0..s.len() {
            // If a repeated character is encountered, remove characters from the start of the window
            // until the repeated character is gone.
            while chars_seen.contains(&s[end..end+1]) {
                chars_seen.remove(&s[start..start+1]);
                start += 1;
            }
            // Add the new character to the set of seen characters.
            chars_seen.insert(&s[end..end+1]);
            // Update max_len if the length of the current window is greater.
            max_len = max_len.max(end - start + 1);
        }
        max_len as i32
}

#[cfg(test)]
mod tests {
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
