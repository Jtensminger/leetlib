/* #TAGS[Hash Table, String, Sorting] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/valid-anagram/] */

        // contains the same letters, no extras, none missing
        // create hashtable of S
        // if all of t chars exist in Hashtable only once
                // if t.char is_in Hashtable && count - 1 < 0, return false
                // in_anagram("cat", "catt")
                // c = 1 - 1 = 0
                // a = 1 - 1 = 0
                // t = 1 - 1 = 0
                // t = 0 - 1 = -1
                // return false
pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false };
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                assert!(is_anagram(String::from("anagram"), String::from("nagaram")));
        }

        #[test]
        fn ext2() {
                assert!(!is_anagram(String::from("rat"), String::from("car")));
        }

        #[test]
        fn ext3() {
                assert!(!is_anagram(String::from("ab"), String::from("a")));
        }

        #[test]
        fn ext4() {
                assert!(!is_anagram(String::from("aacc"), String::from("ccac")));
        }

}