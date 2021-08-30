use std::collections::LinkedList;
use std::convert::TryInto;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // fn push(&mut self, val: i32) {
    //     if self.next == None{
    //         self.next = Some(Box::new(ListNode::new(val)));
    //     }
    //     else{
    //         let inner_node = self.next.as_ref().unwrap();
    //         println!("Inner Node: {:?}", inner_node.next);
    //     }
    // }
}

pub fn construct_link(mut values: Vec<i32>) -> Option<Box<ListNode>> {
    values.reverse();
    let mut i: usize = 1;
    let vec_size = values.len();

    let mut previous_list = Some(Box::new(ListNode::new(values[0])));

    let mut current_list: ListNode;

    while i < vec_size {
        current_list = ListNode {
            val: values[i],
            next: previous_list,
        };
        previous_list = Some(Box::new(current_list));

        i += 1;
    }
    return previous_list
}
pub fn test() {
    let test1_n1 = ListNode::new(2);
    let test1_n2 = ListNode {
        val: 3,
        next: Some(Box::new(test1_n1)),
    };
    let test1_n3 = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(test1_n2)),
    }));

    println!("Node3: {:?}", test1_n3);
}

// pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     // have some carry value that will add to the next value in the linked list
//     // Aporach: get the two cooropsonding vals of l1, l2, add them together, get a carry bit, save that carry bit then
//         // move to next value pair.

//     println!("L1 is: {:?}", l1);
//     println!("L2 is: {:?}", l2);
//     let example_return = ListNode::new(2);
// }
