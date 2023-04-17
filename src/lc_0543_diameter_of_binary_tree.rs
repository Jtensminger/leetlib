
//Definition for a binary tree node.
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

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // get the height of left & right sub-trees
        // diameter = sum of left & right heights
        match root {
                rt => dfs(&rt).1
        }
}

pub fn dfs(sub_tree: &Option<Rc<RefCell<TreeNode>>>) -> (i32,i32) {
        // None == 0
        // Some == Count of levels
        match sub_tree {
                Some(t) => {
                        let (lh, ld) = dfs(&t.borrow().left);
                        let (rh, rd) = dfs(&t.borrow().right);
                        let diameter = lh + rh;
                        let largest_sub_diameter = cmp::max(ld, rd);
                        return (cmp::max(lh, rh) + 1, cmp::max(diameter, largest_sub_diameter))
                },
                None => return (0,0)
        }
}