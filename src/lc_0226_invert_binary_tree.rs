use std::rc::Rc;
use std::cell::{Ref, RefCell};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
        pub val: T,
        pub left: Option<Rc<RefCell<TreeNode<T>>>>,
        pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
        pub fn new(val: T) -> Self {
                TreeNode {
                        val,
                        left: None,
                        right: None
                }
        }
        
        pub fn peek(&self) -> &T {
                &self.val
        }
}



pub fn invert_tree<T> (root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        
        if let Some(node) = root.as_ref() {
                let mut mut_node = node.borrow_mut();
                invert_tree(mut_node.left.clone());
                invert_tree(mut_node.right.clone());
                let mut x = &mut *mut_node;
                std::mem::swap(&mut x.left, &mut x.right);
        }

        root
}



#[cfg(test)]
mod test {

        use super::*;

        #[test]
        fn ext1() {
                let mut root = TreeNode::new(2);

                root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
                
                let inverted = invert_tree(Some(Rc::new(RefCell::new(root))));
                let l = *inverted.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().peek();
                let r = *inverted.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().peek();
                assert_eq!(1, r);
                assert_eq!(3, l);
        }
}