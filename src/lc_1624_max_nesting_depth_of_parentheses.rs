pub fn max_depth(s: String) -> i32 {
        let mut depth: i32 = 0;
        let mut max_depth: i32 = 0;
        for ch in s.chars() {
                depth += match ch {
                        '(' => 1,
                        ')' => -1,
                        _ => 0
                };
                max_depth = max_depth.max(depth);
        }
        max_depth
}

#[cfg(test)]
pub mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(0, max_depth("".to_string()));
                assert_eq!(0, max_depth("A".to_string()));
                assert_eq!(1, max_depth("()".to_string()));
                assert_eq!(1, max_depth("()()".to_string()));
                assert_eq!(2, max_depth("()(()())".to_string()));
                assert_eq!(3, max_depth("(1+(2*3)+((8)/4))+1".to_string()));
                assert_eq!(3, max_depth("(1)+((2))+(((3)))".to_string()));
        }
}
