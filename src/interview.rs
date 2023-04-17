
// Vector
#[allow(dead_code)]
fn vector() {
    // creation
        let mut v = vec![1,2,3]; // Vec::new();
    // length
        v.len(); 
    // indexing & assignment
        v[0] = 1;
        v.insert(1,3);
    // stack operations
        v.push(1); 
        v.pop().unwrap();
}

// Queue
#[allow(dead_code)]
fn deque() {
    // creation
        use std::collections::VecDeque;
        let mut d: VecDeque<u32> = VecDeque::new();
    // length
        d.len(); 
    // indexing & assignment
        d[0] = 1;
        d.insert(1,3);
    // queue operations
        d.push_back(1); 
        d.push_front(1);
        d.pop_back().unwrap();
        d.pop_front().unwrap();
}

// Hashmap
#[allow(dead_code)]
fn hashmap() {
    // creation
        use std::collections::HashMap;
        let mut hm: HashMap<i32, i32> = HashMap::new();
    // length
        hm.len();
    // insert
        hm.insert(1, 1).unwrap();
    // Look up the values associated with some keys.
        hm.get(&1).unwrap();
    // insert a key only if it doesn't already exist
        hm.entry(2).or_insert(2);
}


// Linked List Defintion
    #[allow(dead_code)]
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }
    impl ListNode {
        #[allow(dead_code)]
        fn new(val: i32) -> Self {
            ListNode {
            next: None,
            val
            }
        }
}


// Binary Tree Definition
    use std::rc::Rc;
    use std::cell::RefCell;
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
}