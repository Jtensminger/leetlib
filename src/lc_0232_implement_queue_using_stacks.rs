/* #TAGS[Stack, Queue] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/implement-queue-using-stacks/] */

#[derive(Default)]
pub struct MyQueue {
        stack_back: Vec<i32>,
        stack_front: Vec<i32>,
}

impl MyQueue {
        pub fn new() -> Self {
                Default::default()
        }

        pub fn push(&mut self, x: i32) {
                self.stack_back.push(x);
        }

        pub fn pop(&mut self) -> i32 {
                self.move_back_to_front();
                self.stack_front.pop().unwrap()
        }

        pub fn peek(&mut self) -> i32 {
                self.move_back_to_front();
                *self.stack_front.last().unwrap()
        }

        pub fn empty(&self) -> bool {
                self.stack_front.is_empty() && self.stack_back.is_empty()
        }

        fn move_back_to_front(&mut self) {
                if self.stack_front.is_empty() {
                        while let Some(x) = self.stack_back.pop() {
                                self.stack_front.push(x);
                        }
                }
        }
}


#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let mut q = MyQueue::new();
                q.push(1);
                q.push(2);
                let pop_1 = q.pop();
                let pop_2 = q.pop();
                let is_empty = q.empty();

                assert_eq!(pop_1, 1);
                assert_eq!(pop_2, 2);
                assert_eq!(true, is_empty);
        }
}