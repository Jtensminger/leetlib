/* #TAGS[Binary Tree, Depth-First Search, Recursion] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/invert-binary-tree/] */

use std::rc::Rc;
use std::cell::{RefCell};
// use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
        pub val: T,
        pub left: Link<T>,
        pub right: Link<T>,
}

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;
#[allow(dead_code)]
impl<T> TreeNode<T> {
        pub fn new(val: T) -> Self {
                TreeNode {
                        val,
                        left: None,
                        right: None
                }
        }
        pub fn left(mut self, val: T) -> Self {
                self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                self
        }

        pub fn right(mut self, val: T) -> Self {
                self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                self
        }

        pub fn peek(&self) -> &T {
                &self.val
        }

        // pub fn insert(mut self, val: T) {
        //         let mut queue: VecDeque<Link<T>> = VecDeque::new();
                
        //         queue.push_front(Some(Rc::new(RefCell::new(self))));
        // }
}

fn invert_tree<T> (root: Link<T>) -> Link<T> {
        
        if let Some(node) = root.as_ref() {
                let mut mut_node = node.borrow_mut();
                invert_tree(mut_node.left.clone());
                invert_tree(mut_node.right.clone());
                let x = &mut *mut_node;
                std::mem::swap(&mut x.left, &mut x.right);
        }

        root
}
// [4,2,7,1,3,6,9,0,0,0,0,0,0,0,0]
//              4
//       2           7
//    1     3     6    9
//  0  0  0  0  0  0  0  0

// pub fn tree_to_list<T> (tree: Link<T>) -> Vec<T> {
//         let mut result: Vec<T> = Vec::new();
//         traverse(tree, &mut result);
//         result        
// }

// pub fn traverse<T> (tree: Link<T>, result: &mut Vec<T>) -> Option<bool> {
//         if tree.is_none() { return None }

//         let current_tree = tree.unwrap();
//         let current_value = current_tree.borrow().val;

//         result.push(current_value);
//         traverse(current_tree.to_owned().borrow().left.to_owned(),result);
//         traverse(current_tree.to_owned().borrow().right.to_owned(),result);
        
//         Some(true)
// }

// pub fn list_to_tree<T> (mut list: Vec<T>) -> Link<T> {
//         if list.is_empty() { return None } 
//         if list.len() % 2 == 0 { return None}

//         let mut root = TreeNode::new(list.pop().unwrap())
//                 .left(list.pop().unwrap())
//                 .right(list.pop().unwrap());
        
//         let mut parent_level: VecDeque<&mut Link<T>> = VecDeque::from([&mut root.left, &mut root.right]);

//         while let Some(parent) = parent_level.pop_back().unwrap() {
//                 if let (Some(l), Some(r)) = (list.pop(), list.pop()) {
//                         parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(l))));
//                         parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(r))));
//                 }
//         }


//         Some(Rc::new(RefCell::new(root)))
// }


#[cfg(test)]
mod test {

        use super::*;

        #[test]
        fn ext1() {
                let mut root = TreeNode::new(2);

                root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
                
                let mut inverted = invert_tree(Some(Rc::new(RefCell::new(root))));
                let l = *inverted.as_mut().unwrap().borrow_mut().left.as_ref().unwrap().borrow().peek();
                let r = *inverted.as_mut().unwrap().borrow_mut().right.as_ref().unwrap().borrow().peek();
                assert_eq!(1, r);
                assert_eq!(3, l);
        }
}