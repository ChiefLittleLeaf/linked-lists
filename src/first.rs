// NOTE: Bad singly linked stack

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

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // NOTE: while let == do this thing until the pattern no longer matches
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
            // NOTE: boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node)
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
}

// NOTE: setup tests
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        // NOTE: Create empty list
        let mut list = List::new();

        // NOTE: Check empty list behavior is right
        assert_eq!(list.pop(), None);

        // NOTE: Populate the list
        list.push(1);
        list.push(2);
        list.push(3);

        // NOTE: check removal from list
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // NOTE: add more items to the list to make sure nothing is corrupted
        list.push(4);
        list.push(5);

        // NOTE: check removal again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // NOTE: check exhaustive removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
