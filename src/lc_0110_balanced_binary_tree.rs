/* #TAGS[Tree, Depth-First Search, Binary Tree] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/balanced-binary-tree/] */

// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
                TreeNode {
                        val,
                        left: None,
                        right: None
                }
        }
}


/* 
Definition: Height-balanced...
        A height-balanced binary tree is a binary tree in which
        the depth of the two subtrees of every node
        never differs by more than one
 */

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        height(&root) != -1
}

fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
                Some(node) => {
                        let left = height(&node.borrow().left);
                        let right = height(&node.borrow().right);
                        
                        if i32::abs(left - right) > 1 || left == -1 || right == -1 {
                                return -1
                        }
                        i32::max(left, right) + 1
                },
                None => 0
        }
}

#[cfg(test)]
mod test {
        use super::*;

        #[test] 
        fn ext1() {
                let root = Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 2,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 3,
                                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                                        left: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                                }))),
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 3,
                                        right: None,
                                        left: None
                                })))
                        })))
                })));
                
                // tree is NOT balanced
                assert_eq!(false, is_balanced(root));
        }

        #[test]
        fn ext2() {
                let root = Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 20,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 15,
                                        right: None,
                                        left: None
                                }))),
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 7,
                                        right: None,
                                        left: None
                                })))
                        })))
                })));
                
                // tree is balanced
                assert!(is_balanced(root));
        }
}