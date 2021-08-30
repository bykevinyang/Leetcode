use std::collections::linked_list;

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

pub fn breakdown_linked(linked_list: Option<Box<ListNode>>) {
    let mut next_node = linked_list;
    println!("Next Node: {:?}", next_node);
    loop {
        let unwrapped_linked_list = next_node.unwrap();
        next_node = unwrapped_linked_list.next;
        let node_val = unwrapped_linked_list.val;

        println!("Node Val: {:?}", node_val);
        print!("\n");

        println!("Next Node: {:?}", next_node);

        if next_node == None {
            break;
        }
    }
}

// pub fn test() {
//     let test1_n1 = ListNode::new(2);
//     let test1_n2 = ListNode {
//         val: 3,
//         next: Some(Box::new(test1_n1)),
//     };
//     let test1_n3 = Some(Box::new(ListNode {
//         val: 4,
//         next: Some(Box::new(test1_n2)),
//     }));

//     println!("Node3: {:?}", test1_n3);
// }

pub fn check_len(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
    let mut node1 = l1;
    let mut node2 = l2;

    let mut len1 = 0;
    let mut len2 = 0;

    while node1 != None {
        let linked_list1 = node1.unwrap();
        node1 = linked_list1.next;
        len1 += 1;
    }

    while node2 != None {
        let linked_list2 = node2.unwrap();
        node2 = linked_list2.next;
        len1 += 1;
    }
    
    if len1 == len2 {
        return true 
    }
    else {
        return false 
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    // have some carry value that will add to the next value in the linked list
    // Aporach: get the two cooropsonding vals of l1, l2, add them together, get a carry bit, save that carry bit then
        // move to next value pair.
    let mut node1 = l1;
    let mut node2 = l2;
    println!("Node1: {:?}", node1);
    println!("Node2: {:?}", node2);

    let mut carry = 0;
    
    let mut sums: Vec<i32> = vec![];

    loop {
        let linked_list1 = node1.unwrap(); 
        let linked_list2 = node2.unwrap(); 
        node1 = linked_list1.next;
        node2 = linked_list2.next;

        let node1_val = linked_list1.val;
        let node2_val = linked_list2.val;

        let mut sum = node1_val + node2_val + carry;
        println!("Sum is: {:?}\n", sum);

        if sum > 9 {
            carry = sum % 9;
            sum = 0;
            println!("Carry: {:?}", carry);
        }

        sums.push(sum);
        // Fix this conditioning when lists aren't same length
        if node1 == None || node2 == None {
            break;
        }  
    }

    // while node2 != None {
    //     // Node2 has values to read
    //     let linked_list2 = node2.unwrap();
    //     let node2_val = linked_list2.val;

    //     let mut sum = carry + node2_val;
        
    //     if sum > 9 {
    //         carry = sum % 9;
    //         sum = 0;
    //         println!("Carry: {:?}", carry);
    //     }

    //     sums.push(sum);

    //     node2 = linked_list2.next;
    // }

    // while node1 != None {
    //     // Node1 has values to read
    //     let linked_list1 = node1.unwrap();
    //     let node1_val = linked_list1.val;

    //     let mut sum = carry + node1_val;
        
    //     if sum > 9 {
    //         carry = sum % 9;
    //         sum = 0;
    //         println!("Carry: {:?}", carry);
    //     }

    //     sums.push(sum);
    //     println!("Sum: {:?}", sums);

    //     node1 = linked_list1.next;
    // }

    sums.reverse();
    return construct_link(sums); 
}
