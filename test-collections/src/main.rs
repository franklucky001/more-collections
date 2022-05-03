use linked_stack::Stack;
use linked_queue::Queue;
use singly_linked_list::List;

fn check_split_off(){
    let mut list = List::new();
    for i in 0..10{
        list.push_back(i);
    }
    let second = list.split_off(5);
    let second_vec = vec![5, 6, 7, 8, 9];
    for (it, val) in second.iter().zip(second_vec){
        assert_eq!(it, &val);
    }
}

fn main() {
    // let mut stack = Stack::new();
    // for i in 0..10{
    //     stack.push(i);
    // }
    check_split_off();
    println!("Hello, world!");
}
