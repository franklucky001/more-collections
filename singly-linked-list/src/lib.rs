pub mod single_list {
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::fmt::Debug;
    use std::iter::FusedIterator;
    use std::mem;
    use std::ops::Deref;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
        tail: WeakLink<T>,
        len: usize,
    }

    type Link<T> = Option<Rc<RefCell<Node<T>>>>;
    type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

    #[derive(Debug)]
    struct Node<T> {
        next: Link<T>,
        elem: T,
    }

    impl <T> Node<T>{
        fn new(elem: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node { next: None, elem }))
        }
    }

    impl<T> Default for List<T> {
        fn default() -> Self {
            Self{
                head: None,
                tail: None,
                len: 0
            }
        }
    }
    impl<T> List<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
                len: 0,
            }
        }
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn empty(&self) -> bool {
            self.len == 0
        }
        pub fn push_front(&mut self, elem: T) {
            let new_node = Node::new(elem);
            match self.head.take() {
                Some(head) => {
                    new_node.deref().borrow_mut().next = Some(head);
                    self.head = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(Rc::downgrade(&new_node));
                }
            }
            self.len += 1;
        }
        pub fn front(&self) -> Option<&T> {
            self.head
                .as_ref()
                .map(|head| unsafe { &(*head.deref().as_ptr()).elem })
        }
        pub fn front_mut(&mut self) -> Option<&mut T> {
            self.head
                .as_mut()
                .map(|head| unsafe { &mut (*head.deref().as_ptr()).elem })
        }
        pub fn pop_front(&mut self) -> Option<T> {
            self.pop_front_node().map(|node| node.elem)
        }
        fn pop_front_node(&mut self) -> Option<Node<T>> {
            self.head.take().map(|head| {
                match head.deref().borrow_mut().next.take() {
                    Some(next) => {
                        self.head = Some(next);
                    }
                    _ => {}
                }
                self.len -= 1;
                Rc::try_unwrap(head).ok().unwrap().into_inner()
            })
        }
        pub fn push_back(&mut self, elem: T) {
            let new_node = Node::new(elem);
            match self.tail.take() {
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(Rc::downgrade(&new_node));
                }
                Some(tail) => unsafe {
                    let rc_tail = tail.upgrade().unwrap();
                    (*rc_tail.deref().as_ptr()).next = Some(new_node.clone());
                    self.tail = Some(Rc::downgrade(&new_node));
                },
            }
            self.len += 1;
        }
        pub fn back(&self) -> Option<&T> {
            self.tail
                .as_ref()
                .map(|tail| unsafe { &(*tail.upgrade().unwrap().as_ptr()).elem })
        }
        pub fn back_mut(&mut self) -> Option<&mut T> {
            self.tail
                .as_mut()
                .map(|tail| unsafe { &mut (*tail.upgrade().unwrap().as_ptr()).elem })
        }
        pub fn pop_back(&mut self) -> Option<T> {
            self.pop_back_node().map(|node| node.elem)
        }
        fn pop_back_node(&mut self) -> Option<Node<T>> {
            self.tail.take().map(|tail| unsafe {
                let rc_tail = tail.upgrade().unwrap();
                if self.len == 1 {
                    self.head.take();
                } else {
                    let mut prev = self.head.as_ref();
                    let mut step = self.len.checked_sub(2).unwrap_or(0);
                    while let Some(prev_node) = prev {
                        if step == 0 {
                            break;
                        }
                        step -= 1;
                        prev = Option::from(&(*prev_node.deref().as_ptr()).next);
                    }
                    if let Some(prev_node) = prev {
                        (*prev_node.deref().as_ptr()).next.take();
                        self.tail = Some(Rc::downgrade(prev_node));
                    }
                }
                self.len -= 1;
                Rc::try_unwrap(rc_tail).ok().unwrap().into_inner()
            })
        }
        pub fn append(&mut self, other: &mut Self) {
            match self.tail.take() {
                None => {
                    mem::swap(self, other);
                }
                Some(tail) => {
                    if let Some(other_head) = other.head.take() {
                        let rc_tail = tail.upgrade().unwrap();
                        rc_tail.deref().borrow_mut().next = Some(other_head);
                        self.tail = other.tail.take();
                    }
                }
            }
            self.len += mem::replace(&mut other.len, 0);
        }
        pub fn prepend(&mut self, other: &mut Self) {
            match self.head.take() {
                None => {
                    mem::swap(self, other);
                }
                Some(head) => {
                    if let Some(other_tail) = other.tail.take() {
                        let rc_tail = other_tail.upgrade().unwrap();
                        rc_tail.deref().borrow_mut().next = Some(head);
                        self.head = other.head.take();
                    }
                }
            }
            self.len += mem::replace(&mut other.len, 0);
        }
        pub fn split_off(&mut self, at: usize) -> Self{
            let len = self.len();
            assert!(at <= len, "Cannot split off at a nonexistent index");
            if at == 0 {
                return mem::take(self);
            } else if at == len {
                return Self::new();
            }else{
                let mut prev = self.head.as_ref();
                let mut step = at.checked_sub(1).unwrap_or(0);
                while let Some(prev_node) = prev{
                    if step == 0 {
                        break
                    }
                    unsafe {
                        prev = Option::from(&(*prev_node.deref().as_ptr()).next);
                    }
                    step -= 1;
                }
                let second_head: Link<T> = if let Some(prev_node) = prev{
                    unsafe {
                        (*prev_node.deref().as_ptr()).next.take()
                    }
                }else {
                    None
                };
                let second_tail = self.tail.take();
                let second = Self{
                    head: second_head,
                    tail: second_tail,
                    len: self.len - at
                };
                self.tail = prev.map(|prev_node|{
                    Rc::<RefCell<Node<T>>>::downgrade(prev_node)
                });
                self.len = at;
                second
            }
        }
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                head: self
                    .head
                    .as_ref()
                    .map(|head| unsafe { &*head.deref().as_ptr() }),
                len: self.len,
            }
        }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                head: self
                    .head
                    .as_mut()
                    .map(|head| unsafe { &mut *head.deref().as_ptr() }),
                len: self.len,
            }
        }
    }
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut head = self.head.take();
            while let Some(node) = head {
                if let Ok(node) = Rc::try_unwrap(node) {
                    head = node.into_inner().next;
                } else {
                    break;
                }
            }
        }
    }
    pub struct IntoIter<T>(List<T>);
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop_front()
        }
    }

    impl<T> IntoIterator for List<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self)
        }
    }

    impl<'a, T> IntoIterator for &'a List<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut List<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
    pub struct Iter<'a, T> {
        head: Option<&'a Node<T>>,
        len: usize,
    }

    impl <T> Iter<'_, T> {
        pub fn is_empty(&self) -> bool{
            self.len == 0
        }
    }
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.len == 0 {
                None
            } else {
                self.head.take().map(|head| {
                    self.head = head.next
                        .as_deref()
                        .map(|node| unsafe { &*node.deref().as_ptr() });
                    self.len -= 1;
                    &head.elem
                })
            }
        }
    }
    impl<T> ExactSizeIterator for Iter<'_, T> {}

    impl<T> FusedIterator for Iter<'_, T> {}

    pub struct IterMut<'a, T> {
        head: Option<&'a mut Node<T>>,
        len: usize,
    }

    impl <T> IterMut<'_, T> {
        pub fn is_empty(&self) -> bool{
            self.len == 0
        }
    }
    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.len == 0 {
                None
            } else {
                self.head.take().map(|head| {
                    self.head = head.next
                        .as_deref()
                        .map(|next| unsafe { &mut *next.as_ptr() });
                    &mut head.elem
                })
            }
        }
    }
    impl<T> ExactSizeIterator for IterMut<'_, T> {}

    impl<T> FusedIterator for IterMut<'_, T> {}

    impl<T> Extend<T> for List<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for it in iter {
                self.push_back(it);
            }
        }
    }
    impl<T> FromIterator<T> for List<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            list.extend(iter);
            list
        }
    }
    impl<T, const N: usize> From<[T; N]> for List<T> {
        fn from(arr: [T; N]) -> Self {
            Self::from_iter(arr)
        }
    }

    impl<T: Eq> PartialEq<Self> for List<T> {
        fn eq(&self, other: &Self) -> bool {
            self.len == other.len && self.iter().eq(other)
        }

        fn ne(&self, other: &Self) -> bool {
            self.len != other.len || self.iter().ne(other)
        }
    }

    impl<T: Eq> Eq for List<T> {}

    impl<T: Ord> PartialOrd for List<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.iter().partial_cmp(other)
        }
    }

    impl<T: Ord> Ord for List<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.iter().cmp(other)
        }
    }

    impl<T: Clone> Clone for List<T> {
        fn clone(&self) -> Self {
            self.iter().cloned().collect()
        }

        fn clone_from(&mut self, other: &Self) {
            let mut iter_other = other.iter();
            if self.len() > other.len() {
                self.split_off(other.len());
            }
            for (elem, elem_other) in self.iter_mut().zip(&mut iter_other) {
                elem.clone_from(elem_other);
            }
            if !iter_other.is_empty() {
                self.extend(iter_other.cloned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::single_list::List;
    use std::collections::LinkedList;

    #[test]
    fn basic_back() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.empty(), true);

        // Populate list
        list.push_back(1);
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.back_mut(), Some(&mut 1));
        list.push_back(2);
        assert_eq!(list.back(), Some(&2));
        list.push_back(3);
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.len(), 3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        // 1 4 5
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn basic_front() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list 3- 2 -1
        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.front_mut(), Some(&mut 1));
        list.push_front(2);
        assert_eq!(list.front(), Some(&2));
        list.push_front(3);
        assert_eq!(list.front(), Some(&3));

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        // 5 - 4 - 1
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn check_into_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.into_iter();
        // assert_eq!(list.len(), 0);
        for n in 1..4 {
            assert_eq!(iter.next(), Some(n));
        }
    }
    #[test]
    fn check_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        for n in 1..4 {
            assert_eq!(iter.next(), Some(&n));
        }
    }
    #[test]
    fn check_iter_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        for it in list.iter_mut() {
            *it *= 10;
        }
        let mut iter = list.iter();
        for n in 1..4 {
            let num = n * 10;
            assert_eq!(iter.next(), Some(&num));
        }
    }
    #[test]
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
}

pub use single_list::{IntoIter, Iter, IterMut, List};
