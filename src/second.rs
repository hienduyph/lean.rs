pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, input: T) {
        let node = Box::new(Node {
            elem: input,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match create new branch of execution
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    // or 2018 edition
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|v| &mut **v),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, None);
        while let Some(mut node) = cur_link {
            cur_link = std::mem::replace(&mut node.next, None);
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// 1.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}

// 2. Iter impl
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|v| &**v);
            &node.elem
        })
    }
}

// 3.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|v| &mut **v);
            &mut node.elem
        })
    }
}

mod test {
    #[test]
    fn basic() {
        use super::List;
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peak(), Some(&3));
        assert_eq!(list.peak_mut(), Some(&mut 3));
        list.peak_mut().map(|v| {
            *v = 22;
        });
        assert_eq!(list.peak(), Some(&22));

        assert_eq!(list.pop(), Some(22));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter() {
        use super::List;
        let mut list = List::new();
        list.push(10);
        list.push(2);
        list.push(2);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(10));
    }

    #[test]
    fn iter_mut() {
        use super::List;
        let mut list = List::new();
        list.push(2);
        list.push(3);
        list.push(4);

        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 4));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
    }
}
