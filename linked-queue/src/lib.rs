mod linked_queue{
    use singly_linked_list::List;
    pub struct Queue<T>(List<T>);

    impl <T> Queue<T> {
        pub fn new() -> Self<>{
            Self(List::new())
        }

        pub fn push(&mut self, elem: T){
            self.0.push_back(elem)
        }

        pub fn top(&self) -> Option<&T>{
            self.0.front()
        }
        pub fn top_mut(&mut self) -> Option<&mut T>{
            self.0.front_mut()
        }
        pub fn pop(&mut self) -> Option<T>{
            self.0.pop_front()
        }
        pub fn len(&self) -> usize{
            self.0.len()
        }
        pub fn empty(&self) -> bool{
            self.0.empty()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub use linked_queue::Queue;
