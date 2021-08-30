use crate::Add_Two_Numbers::{ListNode, add_two_numbers, breakdown_linked, construct_link};

mod Add_Two_Numbers;

fn main(){
    // let example_list1 = vec![5, 6, 4];
    // let linked_list1 = construct_link(example_list1);
    // let example_list2 = vec![2, 4, 3];
    // let linked_list2 = construct_link(example_list2);

    // let answer = add_two_numbers(linked_list1, linked_list2);
    // println!("Return: {:?}", answer);


    let list1 = Add_Two_Numbers::construct_link(vec![9, 9, 9, 9, 9, 9, 9]);
    let list2 = Add_Two_Numbers::construct_link(vec![9, 9, 9, 9]);

    let result = Add_Two_Numbers::add_two_numbers(list1, list2);
    println!("Result is: {:?}", result);
    println!("Answer is: {:?}", Add_Two_Numbers::construct_link(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    // breakdown_linked(linked_list);
    // let unwrapped = linked_list.unwrap();
    // let mut node_val = unwrapped.val;
    // let mut next_node = unwrapped.next;
   
    // while next_node != None {
    //     println!("Node: {:?}", node_val);
    //     println!("Next: {:?}", next_node);

    //     next_node = next_node.unwrap().next;
        
    // }
}

