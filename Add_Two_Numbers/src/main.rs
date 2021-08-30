use crate::Add_Two_Numbers::{ListNode, construct_link};

mod Add_Two_Numbers;

fn main(){
    let example_list = vec![5, 6, 4];
    let linked_list = construct_link(example_list);

    println!("{:?}", linked_list);

}
