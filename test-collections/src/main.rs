use linked_stack::Stack;
use linked_queue::Queue;

fn main() {
    let mut stack = Stack::new();
    for i in 0..10{
        stack.push(i);
    }
    println!("Hello, world!");
}
