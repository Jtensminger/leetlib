fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(n as usize);
        let mut stack = Vec::new();

        stack.push((String::new(), n, n));

        while let Some((s, open, close)) = stack.pop() {
                // dbg!(&s);
                // dbg!(open);
                // dbg!(close);
                if open == 0 && close == 0 {
                        res.push(s);
//                        dbg!(&res.last().unwrap());

                } else {
                        if open > 0 {
                                stack.push((s.clone() + "(", open - 1, close));
                        }
                        if close > open {
                                stack.push((s + ")", open, close - 1));
                        }
                }
        }
        res
}
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        generate_parenthesis(3);
    }
}