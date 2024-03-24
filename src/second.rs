// NOTE: This is still a singly linked list
// just more optimized than first.rs linked list.

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// NOTE: No lifetimes here List has no associated lifetimes
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    // NOTE: We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the Iter is around.
    // NOTE: This is the same as the uncommented function with elision lifetimes
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     Iter {
    //         next: self.head.as_deref(),
    //     }
    // }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

// NOTE: Iter is generic over *some* lifetime, it does not care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// NOTE: We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // NOTE: We need one here as well, this is a type declaraction
    type Item = &'a T;

    // NOTE: None of this needs to change, handled by the above.
    // Self continues to be the mvp
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

// NOTE: Tuple stucts are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: access to fields of a tuple struct numerically
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // NOTE: while let == do this thing until the pattern no longer matches
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // NOTE: boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
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

    // NOTE: Test peek functionality
    #[test]
    fn test_peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek_mut(), Some(&mut 42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
