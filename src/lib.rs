use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

type BareTree<T> = Arc<Mutex<Node<T>>>;
type Tree<T> = Option<BareTree<T>>;

#[derive(Debug)]
pub struct Node<T: Ord> {
    pub val: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Clone)]
pub struct BinarySearchTree<T: Ord> {
    root: Tree<T>,
}

impl<T: Ord> Node<T> {
    fn new(val: T) -> Tree<T> {
        Some(Arc::new(Mutex::new(Node {
            val: val,
            left: None,
            right: None,
        })))
    }
}

impl<T: Ord + Clone + std::fmt::Debug> BinarySearchTree<T> {
    pub fn new(val: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new(val),
        }
    }

    pub fn add(&self, val: T) {
        if let Some(root) = &self.root {
            let root_clone = Arc::clone(&root);
            self.add_r(Some(root_clone), val);
        }
    }

    fn add_r(&self, node: Tree<T>, val: T) -> (Tree<T>, BareTree<T>) {
        if let Some(n) = node {
            let new: BareTree<T>;
            let mut node_guard = n.lock().unwrap();
            let current_val = node_guard.val.clone();
            if &current_val <= &val {
                let left = node_guard.left.clone();
                let new_tree = self.add_r(left, val);
                new = new_tree.1;
                let new_tree = new_tree.0.unwrap();
                node_guard.left = Some(new_tree);
            } else {
                let right = node_guard.right.clone();
                let new_tree = self.add_r(right, val);
                new = new_tree.1;
                let new_tree = new_tree.0.unwrap();
                node_guard.right = Some(new_tree);
            }
            drop(node_guard);
            (Some(n), new)
        } else {
            let new = Node::new(val);
            (new.clone(), new.unwrap())
        }
    }

    pub fn find(&self, val: T) -> Tree<T> {
        self.find_r(&self.root, &val)
    }

    fn find_r(&self, node: &Tree<T>, val: &T) -> Tree<T> {
        match node {
            Some(n) => {
                let n_guard = n.lock().unwrap();
                match n_guard.val.cmp(&val) {
                    Ordering::Less => self.find_r(&n_guard.left, val),
                    Ordering::Equal => Some(Arc::clone(n)),
                    Ordering::Greater => self.find_r(&n_guard.right, val),
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&T) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree<T>, callback: &impl Fn(&T) -> ()) {
        if let Some(n) = node {
            let n = n.lock().unwrap();

            self.walk_in_order(&n.left, callback);
            callback(&n.val);
            self.walk_in_order(&n.right, callback);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BinarySearchTree;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn insert_many() {
        const N: usize = 300;

        let tree = Arc::new(BinarySearchTree::new(0));

        // spawn N-1 threads to add N-1 elements to the tree (0 already in tree)
        let mut handles = vec![];
        for i in 1..N {
            handles.push(thread::spawn({
                let tree_clone = Arc::clone(&tree);
                move || {
                    tree_clone.add(i);
                }
            }));
        }

        for handle in handles {
            let _ = handle.join().unwrap();
        }

        // check that all N elements exist in tree
        for i in 0..N {
            let found = tree.find(i);
            assert_eq!(true, found.is_some());
            let node = found.unwrap();
            let val = node.lock().unwrap().val;
            assert_eq!(val, i);
        }
    }
}
