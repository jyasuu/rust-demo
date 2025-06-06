
use std::{mem, ops::{Deref, DerefMut}};

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    
    pub fn peek(&self) -> Option<&i32> {
        match &self.head {
            Link::Empty => None,
            Link::More(node) => 
            {
                Some(&node.elem)
            },
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> {
        match &mut self.head {
            Link::Empty => None,
            Link::More(node) => 
            {
                Some(&mut node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


pub struct IntoIter(List);

impl List {
    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}


pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl List {
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        let next = match &self.head {
            Link::Empty => None,
            Link::More(node) => Some(node.deref()),
        };
        Iter { next }
    }
}

impl List {
    pub fn iter2(&self) -> Iter {
        let next = match &self.head {
            Link::Empty => None,
            Link::More(node) => Some(node.deref()),
        };
        Iter { next }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            let next = match &node.next {
                Link::Empty => None,
                Link::More(local_node) => Some(local_node.deref()),
            };
            self.next = next;
            &node.elem
        })
    }
}




pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

impl List {
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        
        let next = match &mut self.head {
            Link::Empty => None,
            Link::More(node) => Some(node.deref_mut()),
        };

        IterMut { next}
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        
        self.next.take().map(|node| {
            
            let next = match &mut node.next {
                Link::Empty => None,
                Link::More(node) => Some(node.deref_mut()),
            };
            
            self.next = next;
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }


    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        let mut sec = iter.next();
        assert_eq!(sec, Some(&mut 2));
        match sec.as_mut() {
            Some(v) => **v = 4,
            None => {},
        }
        assert_eq!(iter.next(), Some(&mut 1));

        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&1));
    }

}

fn main()
{
    
}