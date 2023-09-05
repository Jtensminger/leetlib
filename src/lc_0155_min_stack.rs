struct MinStack {
        stack: Vec<i32>
}

/*
         * `&self` means the method takes an immutable reference.
         * If you need a mutable reference, change it to `&mut self` instead.
*/

impl MinStack {

        pub fn new() -> Self {
                MinStack {
                        stack: Vec::new()
                }
        }
        
        pub fn push(&mut self, val: i32) {
                self.stack.push(val);
        }
        
        pub fn pop(&mut self) {
                self.stack.pop().unwrap();
        }
        
        pub fn top(&self) -> i32 {
                *self.stack.last().unwrap()
        }
        
        pub fn get_min(&self) -> i32 {
                *self.stack.iter().min().unwrap()
        }
}

/*
         * Your min_stack object will be instantiated and called as such:
         * let obj = min_stack::new();
         * obj.push(val);
         * obj.pop();
         * let ret_3: i32 = obj.top();
         * let ret_4: i32 = obj.get_min();
*/

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                let mut min_stack = MinStack::new();
                min_stack.push(-2);
                min_stack.push(0);
                min_stack.push(-3);
                assert_eq!(min_stack.stack, vec![-2, 0, -3]);
                assert_eq!(min_stack.get_min(), -3);
                min_stack.pop();
                assert_eq!(min_stack.stack, vec![-2, 0]);
                assert_eq!(min_stack.top(), 0);
                assert_eq!(min_stack.get_min(), -2);
        }
}
