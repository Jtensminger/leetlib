/* 
Problem:
        Given an array of strings strs, group the anagrams together. You can return the answer in any order.

        An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
        typically using all the original letters exactly once.
Constraints:
        1 <= strs.length <= 104
        0 <= strs[i].length <= 100
        strs[i] consists of lowercase English letters.

Algo:
        for a list of anagrams,  find each anagrams corresponding anagram(s).
        group the correspondents in a vector.
        return all vector groups
*/

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        // hashmap of the sorted version of each string
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

        // hash each string <AND> store original string alongside it
        for i in strs {
                let mut key: [u8; 26] = [0; 26];
                i.chars().for_each(|c| key[c as usize - 'a' as usize] += 1 );
                
                match map.get_mut(&key) {
                        Some(v) => { v.push(i); }
                        _ => { map.insert(key, vec![i]); }
                }        
        }
        
        // convert hashmap into Vec<Vec<String>>
        map.into_values().collect()
}

#[cfg(test)]
pub mod tests {
        use super::*;
        #[test]
        fn ext1() {
                
                let ungrouped: Vec<String> = to_vs(vec!["eat","tea","tan","ate","nat","bat"]);
                let target_groupings: Vec<Vec<String>> = vec![
                        to_vs(vec!["bat"]),
                        to_vs(vec!["nat","tan"]),
                        to_vs(vec!["ate","eat","tea"])
                ];
                let grouped = group_anagrams(ungrouped);
                for mut i in grouped {
                        i.sort();
                        assert_eq!(true, target_groupings.contains(&i));
                }
        }

        fn to_vs(v: Vec<&str>) -> Vec<String> {
                v.iter()
                .map(|&s| String::from(s))
                .collect()
        }
}