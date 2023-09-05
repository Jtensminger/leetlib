/*
Problem: 
        Write a function to find the longest common prefix string amongst an array of strings. 
        If there is no common prefix, return an empty string "".
Constraints:
        1 <= strs.length <= 200
        0 <= strs[i].length <= 200
        strs[i] consists of only lowercase English letters.
 */

 fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
                return String::from("");
        }
    
        let shortest_word_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let first_word = &strs[0];
    
        (0..shortest_word_len)
                .take_while(|&idx| {
                        let ch = first_word.chars().nth(idx).unwrap();
                        strs.iter().all(|word| word.chars().nth(idx).unwrap() == ch)
                 })
                .map(|idx| first_word.chars().nth(idx).unwrap())
                .collect()
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn it_works() {
                let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
                assert_eq!(String::from("fl"), longest_common_prefix(strs));
        }
}
