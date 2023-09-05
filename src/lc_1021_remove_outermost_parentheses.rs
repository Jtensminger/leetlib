fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::with_capacity(s.len());        
        let mut depth = 0;
        for ch in s.chars() {
                let prev_depth = depth;
                depth += if ch == '(' { 1 } else { -1 };
                /* prevents pushing outer (); NOT 0 -> 1 or 1 -> 0 */
                if prev_depth | depth != 1 { 
                        res.push(ch);
                }
        }
        res
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!("", remove_outer_parentheses("()".to_string()));
                assert_eq!("", remove_outer_parentheses("()()".to_string()));
                assert_eq!("()", remove_outer_parentheses("(())".to_string()));
                assert_eq!("()()()", remove_outer_parentheses("(()())(())".to_string()));
        }
}
