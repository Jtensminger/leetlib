/* 
node properties:
    * unique
    * decendent of themselves

    BST Definition: (left < parent < right) Recursively
        * The left subtree of a node contains only nodes with keys lesser than the node’s key.
        * The right subtree of a node contains only nodes with keys greater than the node’s key.
        * The left and right subtree each must also be a binary search tree.


    1. find node 1
    2. find node 2
    
    Simplest case of LCA:
    diagram: 2 -> 1 
    root: 2
    lowest_common_ancestor(root, p, q) -> node:
        // traverse
            
        if root.is_none:
            return None

        if root > p && root > q:
            return LCA(root.left, p, q)
        
        if root < p && root < q:
            return LCA(root.right, p, q)
            
        return root;
    
*/

// Definition for a binary tree node.
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
pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut lo = p.unwrap().borrow().val;
        let mut hi = q.unwrap().borrow().val;
        
        if lo > hi {
                let tmp = lo;
                lo = hi;
                hi = tmp;
        }

        println!("lo {lo:?}");

        while let Some(n) = root {
                let v = n.borrow().val;
                if v == hi || v == lo || (hi > v && v > lo) {
                        return Some(n);
                }
                if v > hi {
                        root = n.borrow().left.clone();
                } else {
                        root = n.borrow().right.clone();
                }
        }
        root
}




mod test {
        use super::*;

        #[test]
        fn ext1() {
                
        }

}