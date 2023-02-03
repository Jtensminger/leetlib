// TODO: Submission Code: Merge Two Sorted Lists
// TODO: How to measure time & memory performance locally?

/* 
        You are given the heads of two sorted linked lists: list1 and list2.

        Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

        Return the head of the merged linked list.
 */

 // Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
}

impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
                ListNode {
                        next: None,
                        val
                }
        }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // L1: 1 -> 2 -> 4
        // L2: 2 -> 3 -> 4
        // Output: 1 -> 2 -> 2 -> 3 -> 4 -> 4

        match (list1, list2) {
                (None, None) => None, // return empty linked list

                (Some(n), None) | (None, Some(n)) => Some(n), // return the only node in the two lists

                (Some(l1), Some(l2))  => {
                        if l1.val >= l2.val {
                                Some(Box::new(ListNode {
                                        val: l2.val,
                                        next: Some(l1)
                                }))
                        } else {
                                Some(Box::new(ListNode {
                                        val: l1.val,
                                        next: Some(l2)
                                }))
                        }
                }
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {

                let mut l2 = Some(Box::new(ListNode::new(2)));

                let mut l3 = Some(Box::new(ListNode { val: 3, next: l2 }));

                let mut l1 = Some(Box::new(ListNode::new(1)));

                let merged = merge_two_lists(l3, l1);
                
                println!("{}", merged.unwrap().val);
                assert_eq!(true, true);
        }
}


/*  
        #[cfg(test)]
        use super::*;

        #[test]
        #[ignore]
        #[should_panic]
        #[should_panic(expected = "custom error message")] 

        fn ext1() { Ok(()) }
        fn ext1() { Err(String::from("Error Message")) }

        assert!(true);            // succeeds
        assert_eq!(true, true);   // succeeds
        assert_ne!(false, true);  // succeeds

        eprintln("Error: Could not complete task");      // prints to the standard error
*/