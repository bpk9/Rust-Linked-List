use std::mem;

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

    pub fn size(&self) -> u32 {
        let mut current = &self.head;
        let mut count = 0;

        loop {
            match current {
                Link::Empty => return count,
                Link::More(node) => {
                    current = &node.next;
                    count += 1;
                }
            }
        };
    }

    pub fn elem_at(&mut self, idx: i32) -> Option<i32> {
        let mut current = &self.head;
        let mut count = 0;

        loop {
            match current {
                Link::Empty => return None,
                Link::More(node) => {
                    if count == idx {
                        return Some(node.elem);
                    }

                    current = &node.next;
                    count += 1;
                }
            }
        };
    }

    pub fn insert(&mut self, idx: i32, elem: i32) {
        // implement this function for problem 4
        unimplemented!()
    }

    pub fn delete(&mut self, idx: i32) {
        // implement this function for problem 5
        unimplemented!()
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
    fn size() {
        let mut list = List::new();
        // Check size of empty list
        assert_eq!(list.size(), 0);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check size
        assert_eq!(list.size(), 3);

        list.pop();
        list.pop();
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn elem_at() {
        let mut list = List::new();

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check elem_at
        assert_eq!(list.elem_at(0), Some(3));
        assert_eq!(list.elem_at(1), Some(2));
        assert_eq!(list.elem_at(2), Some(1));

        list.pop();
        list.pop();
        assert_eq!(list.elem_at(1), None);
        list.pop();
        assert_eq!(list.elem_at(0), None);
    }

    #[test]
    fn insert() {
        let mut list = List::new();

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check insert
        list.insert(2, 4);
        // list: 3 -> 2 -> 4 -> 1
        assert_eq!(list.size(), 4);
        assert_eq!(list.elem_at(0), Some(3));
        assert_eq!(list.elem_at(1), Some(2));
        assert_eq!(list.elem_at(2), Some(4));
        assert_eq!(list.elem_at(3), Some(1));
    }

    #[test]
    fn delete() {
        let mut list = List::new();

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check delete
        list.delete(1);
        // list: 3 -> 1
        assert_eq!(list.size(), 2);
        assert_eq!(list.elem_at(0), Some(3));
        assert_eq!(list.elem_at(1), Some(1));
    }
}
