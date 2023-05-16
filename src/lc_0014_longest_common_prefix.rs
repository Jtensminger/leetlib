/*
Problem: 
        Write a function to find the longest common prefix string amongst an array of strings. 
        If there is no common prefix, return an empty string "".
Constraints:
        1 <= strs.length <= 200
        0 <= strs[i].length <= 200
        strs[i] consists of only lowercase English letters.
 */

pub fn longest_common_prefix(strs: Vec<String>) -> String {
        /* walk all strings at the same time via indexing */
        if strs.is_empty() {
                return String::from("");
        }
        let mut stack = Vec::with_capacity(strs.len());
        let mut common_prefix = vec![];
        for idx in 0..strs[0].len() {
                for word in &strs {
                        if idx < word.len() {
                                stack.push(word.chars().nth(idx).unwrap());
                            } else {
                                // If a string is shorter than the first string, break out of the loop
                                return common_prefix.iter().collect::<String>();
                            }                }
                let letter = stack[0] as char;
                if stack.iter().all(|&x| x == letter) {
                        common_prefix.push(letter);
                        stack.clear(); 
                } else {
                        break
                }
        }
        common_prefix.iter().collect::<String>()
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
