/*
Given two strings s and t of lengths m and n respectively, return the minimum window substring of s
such that every character in t (including duplicates) is included in the window. 
If there is no such substring, return the empty string "".
*/

/* min window of S that includes every character of T (including duplicates) */

pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() || s.len() < t.len() {
                return "".to_string();
        }
        let s = s.chars().collect::<Vec<char>>();
        /* will optimize this to just be 52 instead of 256 */
        let mut window_count = [0 as i32; 128]; 
        let mut t_count = [0 as i32; 128];
        /* create char frequency counts for t & # of unique characters  */
        let (mut have, mut need) = (0, 0);
        t.chars().into_iter().for_each(|ch| {
                t_count[ch as usize] += 1;
                if t_count[ch as usize] == 1 {
                        need += 1;
                }
        });
        /* sliding window strategy while keeping track of valid substrings */
        let (mut start, mut res_len, mut res) = (0, usize::MAX, (-1 as i32, -1 as i32));
        for end in 0..s.len() {
                let end_ascii = s[end] as usize;
                window_count[end_ascii] += 1;
                // if end exists in T && the counts are the same, increment have
                have += (window_count[end_ascii] == t_count[end_ascii]) as i32;
                while have == need {
                        // update our result
                        if (end - start + 1) < res_len {
                                res = (start as i32, end as i32);
                        }
                        res_len = res_len.min(end - start + 1);
                        // remove start from our window
                        let start_ascii = s[start] as usize;
                        window_count[start_ascii] -= 1;
                        if window_count[start_ascii] < t_count[start_ascii] {
                                have -= 1;
                        }
                        // shrink window by incrementing start
                        start += 1;
                }
        }
        let (l, r) = res;
        if l > -1 && r > -1 {
                return s[l as usize..=r as usize].iter().collect()
        }
        "".to_string()
}

#[cfg(test)]
pub mod tests {
        use super::*;
        /*
                "a" = 97 "b" = 98
                "A" = 65
                "z" = 122 "y" = 121
                "Z" = 90
        */
        #[test]
        fn ext1() {
                let (s, t) = ("aa".to_string(), "aa".to_string());
                assert_eq!("aa".to_string(), min_window(s, t));
        }

        #[test]
        fn ext2() {
                let (s, t) = ("a".to_string(), "aa".to_string());
                assert_eq!("".to_string(), min_window(s, t));
        }

        #[test]
        fn ext3() {
                let (s, t) = ("ADOBECODEBANC".to_string(), "ABC".to_string());
                assert_eq!("BANC".to_string(), min_window(s, t));
        }
}
