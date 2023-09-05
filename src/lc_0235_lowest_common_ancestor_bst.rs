/* #TAGS[Binary Search Tree, Recursion, Binary Tree] #DIFFICULTY[Medium] #URL[https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/] */

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
struct TreeNode {
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

#[allow(dead_code)]
fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;   
        let q_val = q.unwrap().borrow().val;
        lca_recursive(&root, p_val, q_val)
}

#[allow(dead_code)]
fn lca_recursive(root: &Option<Rc<RefCell<TreeNode>>>, lower: i32, higher: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if lower > higher {
                return lca_recursive(root, higher, lower);
        }

        match root {
                Some(cell) => {
                        let val = cell.borrow().val;

                        if higher < val {
                                lca_recursive(&cell.borrow().left, lower, higher)
                        } else if lower > val {
                                lca_recursive(&cell.borrow().right, lower, higher)
                        } else {
                                Some(Rc::new(RefCell::new(TreeNode::new(val))))
                        }
                },
                None => None,
        }
}




mod test {

        #[test]
        fn ext1() {
                
        }

}