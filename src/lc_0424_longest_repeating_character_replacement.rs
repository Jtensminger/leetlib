/* Problem:
        You are given a string s and an integer k.
        You can choose any character of the string and change it to any other uppercase English character.
        You can perform this operation at most k times.
        Return the length of the longest substring containing the same letter you can get after performing the above operations.
*/
pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().into_iter().collect();
        let mut counts = [0; 26];
        let (mut max_sub, mut max_ch, mut start) = (0, 0, 0);
        for end in 0..s.len() {
                /* increment char frequency & store most frequent character in window */
                let ascii = s[end] as usize - 'A' as usize;
                counts[ascii] += 1; 
                max_ch = max_ch.max(counts[ascii]);    
                /* determine # of replacements needed to make the window all the same character */
                /* false => invalid window => shrink window */
                while (end - start + 1) - max_ch as usize > k as usize { 
                        let ascii = s[start] as usize - 'A' as usize;
                        counts[ascii] -= 1;
                        start += 1;
                }
                max_sub = max_sub.max(end - start + 1);
        }
        max_sub as i32
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(4, character_replacement("ABAB".to_string(), 2));
                assert_eq!(4, character_replacement("AABABBA".to_string(), 1));
                assert_eq!(4, character_replacement("AAAA".to_string(), 2));
                assert_eq!(4, character_replacement("ABBB".to_string(), 2));
        }
}