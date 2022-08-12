use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Result},
    ops::{Deref, DerefMut},
};

struct Node<T> {
    data: Box<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bst<T: Ord> {
    root: Option<Node<T>>,
}

impl<T: Ord> Bst<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, item: T) {
        match &mut self.root {
            n @ None => *n = Some(Node::new(item)),
            Some(root) => {
                let mut node = root;

                loop {
                    if &item < &node.data {
                        match &mut node.left {
                            n @ None => {
                                *n = Some(Box::new(Node::new(item)));
                                return;
                            }
                            Some(n) => {
                                node = n.deref_mut();
                            }
                        }
                    } else {
                        match &mut node.right {
                            n @ None => {
                                *n = Some(Box::new(Node::new(item)));
                                return;
                            }
                            Some(n) => {
                                node = n.deref_mut();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, item: T) -> Option<T> {
        match &mut self.root {
            None => None,
            Some(root) => {
                let mut node = root;

                loop {
                    if &item < &node.data {
                        match &mut node.left {
                            None => {
                                return None;
                            }
                            Some(n) => {
                                if n.data.deref() == &item {
                                    break;
                                }
                                node = n.deref_mut();
                            }
                        }
                    } else {
                        match &mut node.right {
                            None => {
                                return None;
                            }
                            Some(n) => {
                                if n.data.deref() == &item {
                                    break;
                                }
                                node = n.deref_mut();
                            }
                        }
                    }
                }

                None
            }
        }
    }

    fn _balance(&mut self) {
        todo!()
    }

    pub fn contains(&self, item: T) -> bool {
        match &self.root {
            None => false,
            Some(node) => {
                let item = Node::new(item);
                let mut node = node;

                while node != &item {
                    if &item < node {
                        match &node.left {
                            None => return false,
                            Some(n) => {
                                node = n.deref();
                            }
                        }
                    } else {
                        match &node.right {
                            None => return false,
                            Some(n) => {
                                node = n.deref();
                            }
                        }
                    }
                }
                true
            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => {
                let mut node = node;
                while let Some(n) = &node.left {
                    node = n.deref();
                }
                Some(node.data.deref())
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => {
                let mut node = node;
                while let Some(n) = &node.right {
                    node = n.deref();
                }
                Some(node.data.deref())
            }
        }
    }
}

impl<T: Default + Ord> Default for Bst<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: Box::new(data),
            left: None,
            right: None,
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(
                f,
                "Node {{ data: {:#?}, left: {:#?}, right: {:#?} }}",
                self.data, self.left, self.right
            )
        } else {
            write!(
                f,
                "Node {{ data: {:?}, left: {:?}, right: {:?} }}",
                self.data, self.left, self.right
            )
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node {
            data: Default::default(),
            left: None,
            right: None,
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.data == *other.data
    }
}

impl<T: Eq> Eq for Node<T> {}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        assert!(Bst::<u8>::new() == Bst { root: None });
    }

    #[test]
    fn insert_one() {
        let mut bst = Bst::new();
        bst.insert(0);

        assert!(bst.contains(0));
        assert!(
            bst == Bst {
                root: Some(Node::new(0))
            }
        );
    }

    #[test]
    fn insert_two() {
        let mut bst = Bst::new();

        bst.insert(1);
        assert!(bst.contains(1));
        bst.insert(0);
        assert!(bst.contains(1));
        assert!(bst.contains(0));

        assert!(
            bst == Bst {
                root: Some(Node {
                    data: Box::new(RefCell::new(1)),
                    left: Some(Box::new(Node::new(0))),
                    right: None,
                })
            }
        );
    }
}
