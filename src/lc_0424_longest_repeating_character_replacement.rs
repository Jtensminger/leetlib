/* Problem:
        You are given a string s and an integer k.
        You can choose any character of the string and change it to any other uppercase English character.
        You can perform this operation at most k times.
        Return the length of the longest substring containing the same letter you can get after performing the above operations.
*/
use std::collections::HashMap;
pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut res, mut l, mut maxf) = (0, 0, 0);
        let mut count: HashMap<char, u64> = HashMap::new();
        for r in 0..s.len() {
            *count.entry(s[r]).or_default() += 1;
            maxf = maxf.max(*count.get(&s[r]).unwrap());

            while (r - l + 1) - maxf as usize > k as usize {
                *count.get_mut(&s[l]).unwrap() -= 1;
                l += 1;
            }
            res = res.max(r - l + 1);
        }
        res as i32
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                // assert_eq!(4, character_replacement("ABAB".to_string(), 2));
                assert_eq!(4, character_replacement("AABABBA".to_string(), 1));
                // assert_eq!(4, character_replacement("AAAA".to_string(), 2));
                // assert_eq!(4, character_replacement("ABBB".to_string(), 2));
        }
}