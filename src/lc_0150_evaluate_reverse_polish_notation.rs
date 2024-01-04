use std::ops::{Add, Sub, Mul, Div};
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for i in 0..tokens.len() {            
                match &*tokens[i] {
                        "+" | "-" | "*" | "/" => {
                                let second: i32 = stack.pop().unwrap();
                                let first: i32 = stack.pop().unwrap();
                                
                                let op = match &*tokens[i] {
                                        "+" => i32::add,
                                        "-" => i32::sub,
                                        "*" => i32::mul,
                                        "/" => i32::div,
                                        _ => unreachable!(),
                                };
                                stack.push(op(first, second));
                        }
                        num_string => {
                                stack.push(num_string.parse::<i32>().unwrap());
                        }
                }
        }
        stack.pop().unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn ext1() {
        let tokens: Vec<String> = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]; 
        assert_eq!(9, eval_rpn(tokens))
    }
}
