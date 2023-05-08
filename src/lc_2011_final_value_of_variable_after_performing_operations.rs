pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;

        for i in operations  {
            match &i[..] {
                "++X" | "X++" => x += 1,
                _ => x -=1,
            }
        }
        x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext1() {
        let ops = vec![
            String::from("--X"),
            String::from("X++"),
            String::from("X++")
        ];

        assert_eq!(1, final_value_after_operations(ops));
    }
}
