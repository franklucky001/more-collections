mod linked_stack {
    use singly_linked_list::List;

    pub struct Stack<T>(List<T>);
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self(List::new())
        }
        pub fn push(&mut self, elem: T) {
            self.0.push_front(elem)
        }
        pub fn top(&self) -> Option<&T> {
            self.0.front()
        }
        pub fn top_mut(&mut self) -> Option<&mut T> {
            self.0.front_mut()
        }
        pub fn pop(&mut self) -> Option<T> {
            self.0.pop_front()
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn empty(&self) -> bool {
            self.0.empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;
    #[test]
    fn hello() {
        assert_eq!(1, 1);
        println!("hello world");
    }
}

pub use linked_stack::Stack;
