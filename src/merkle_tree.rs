use std::collections::{BTreeMap, VecDeque};
use std::fmt::Display;
use std::rc::Rc;

use element::Element;
use hash_utils::*;

enum ProofNode<'a> {
    Left(&'a String),
    Right(&'a String)
}

pub struct MerkleTree<T: ToString + Display + Clone> {
    root: Element<T>,
    height: usize,
    count: usize,
    storage: VecDeque<Rc<T>>,
    nodes: BTreeMap<usize, VecDeque<Element<T>>>
}

impl<T: ToString + Display + Clone> MerkleTree<T> {
    /// Creates new, empty `MerkelTree`.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let tree = MerkelTree::new();
    /// assert_eq!(0, tree.len());
    /// ```
    pub fn new() -> Self {
        MerkleTree {
            root: Element::empty(),
            height: 0,
            count: 0,
            storage: VecDeque::new(),
            nodes: BTreeMap::new()
        }
    }

    /// Creates `MerkelTree` from `Vec` of elements.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let values = vec![1, 2, 3, 4];
    /// let tree = MerkelTree::from_vec(values);
    /// assert_eq!(4, tree.len());
    /// ```
    pub fn from_vec(data: Vec<T>) -> Self {
        if data.is_empty() {
            Self::new()
        } else {
            let elements = data.into_iter().map(|e| Rc::new(e)).collect::<VecDeque<Rc<T>>>();
            let mut result = MerkleTree {
                root: Element::empty(),
                height: 0,
                count: 0,
                storage: elements,
                nodes: BTreeMap::new()
            };
            result.calculate_tree();
            result
        }
    }

    /// Push element into the end of the tree.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// tree.push(1);
    /// assert_eq!(1, tree.len());
    /// ```
    pub fn push(&mut self, value: T) {
        self.storage.push_back(Rc::new(value));
        self.count = self.storage.len();
        self.calculate_tree();
    }

    /// Removes element from the tree and returns `true` if element was removed
    /// successfully and `false` if `index` out of bounds.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::from_vec(vec![1, 2, 3]);
    /// assert!(tree.remove(1));
    /// assert_eq!(2, tree.len());
    /// assert!(!tree.remove(5));
    /// ```
    pub fn remove(&mut self, index: usize) -> bool {
        if let Some(_) = self.storage.remove(index) {
            self.count = self.storage.len();
            self.calculate_tree();
            true
        } else {
            false
        }
    }

    /// Retrieves an element in the `MerkelTree` by index.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// tree.push(1);
    /// tree.push(2);
    /// tree.push(3);
    /// assert_eq!(tree.get(1), Some(&2));
    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(v) = self.storage.get(index) {
            Some(v.as_ref())
        } else {
            None
        }
    }

    /// Retrieves copies of all elements in the `MerkelTree`.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// tree.push(1);
    /// tree.push(2);
    /// tree.push(3);
    /// assert_eq!(tree.get_values(), Some(vec![1, 2, 3]));
    pub fn get_values(&self) -> Option<Vec<T>> {
        if self.storage.is_empty() {
            None
        } else {
            let values = self.storage.iter()
                .map(|v| v.as_ref().clone())
                .collect::<Vec<T>>();
            Some(values)
        }
    }

    /// Returns the number of elements in the three
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut mt = MerkelTree::new();
    /// assert_eq!(mt.len(), 0);
    /// v.push(1);
    /// assert_eq!(mt.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns the height of the three
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// assert_eq!(tree.height(), 0);
    /// v.push(1);
    /// assert_eq!(tree.height(), 0);
    /// v.push(2);
    /// assert_eq!(tree.height(), 1);
    /// v.push(3);
    /// assert_eq!(tree.height(), 2);
    ///
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns `true` if the `MerkelTree` is empty.
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// assert!(tree.is_empty());
    /// v.push(1);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }


    /// Returns root hash of `MerkelTree`
    /// # Examples
    ///
    /// root hash of empty tree is: "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::new();
    /// assert_eq!(Some(&"5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9".to_string()), tree.root_hash());
    /// tree.append(1);
    /// tree.append(2);
    /// tree.append(3);
    /// tree.append(4);
    /// assert_eq!(Some(&"85df8945419d2b5038f7ac83ec1ec6b8267c40fdb3b1e56ff62f6676eb855e70".to_string()), tree.root_hash());
    /// ```
    pub fn root_hash(&self) -> Option<&String> {
        self.root.hash()
    }

    /// Verifies if the `value` really presents in `MerkelTree`
    /// # Examples
    ///
    /// ```
    /// extern crate merkel_tree;
    /// use merkel_tree::MerkelTree;
    ///
    /// let mut tree = MerkelTree::from_vec(vec![1, 2, 3, 4]);
    /// assert!(tree.validate_element(2, "85df8945419d2b5038f7ac83ec1ec6b8267c40fdb3b1e56ff62f6676eb855e70");
    /// ```
    pub fn validate_element(&self, value: T, root_hash: &str) -> bool {
        let needed_hashes = self.get_needed_hashes(&value);
        let mut hash = create_leaf_hash(&value);
        let mut level = self.height;

        while level > 0 {
            if let Some(h) = needed_hashes.get(&level) {
                hash = match *h {
                    ProofNode::Left(ref proof_hash) => create_node_hash(proof_hash, &&hash),
                    ProofNode::Right(ref proof_hash) => create_node_hash(&&hash, proof_hash)
                };
            } else {
                return false;
            }
            level -= 1;
        }

        hash == root_hash
    }

    fn calculate_tree(&mut self) {
        self.count = self.storage.len();
        self.height = calculate_height(self.count);
        self.root = Element::empty();
        self.nodes.clear();
        let mut current_level = self.height;

        if !self.storage.is_empty() {
            let mut leaves = VecDeque::new();
            for value in &self.storage {
                let e = Element::create_leaf(value.clone());
                leaves.push_back(e);
            }

            self.nodes.insert(current_level, leaves);

            while current_level > 0 {
                let above_level = current_level - 1;
                let above_row = {
                    let mut row = VecDeque::new();
                    let current_row = self.nodes.get(&current_level).unwrap();
                    for i in (0..current_row.len()).step_by(2) {
                        let left = current_row.get(i).unwrap();
                        let right = current_row.get(i+1).unwrap_or(left);
                        let node = Element::create_node(left.clone(), right.clone());
                        row.push_back(node);
                    }
                    row
                };

                self.nodes.insert(above_level, above_row);
                current_level -= 1;
            }
            assert!(current_level == 0);
            self.root = self.nodes.get(&0).unwrap()[0].clone(); //root_node;
        }
    }

    fn get_needed_hashes(&self, value: &T) -> BTreeMap<usize, ProofNode> {
        let mut level = self.height;
        let mut next_hash = create_leaf_hash(&value);
        let mut needed_hashes = BTreeMap::new();

        while level > 0 {
            if let Some(index) = self.get_element_index(level, &next_hash) {
                let nodes = self.nodes.get(&level).unwrap();
                match nodes.get(index) {
                    Some(&Element::Leaf { ref hash, ..}) | Some(&Element::Node { ref hash, ..}) => {
                        if index % 2 == 0 {
                            if let Some(sibling_node) = nodes.get(index+1) {
                                needed_hashes.insert(level, ProofNode::Right(sibling_node.hash().unwrap()));
                                next_hash = create_node_hash(hash, sibling_node.hash().unwrap());
                            } else {
                                needed_hashes.insert(level, ProofNode::Right(hash));
                                next_hash = create_node_hash(hash, hash);
                            }
                        } else {
                            if let Some(sibling_node) = nodes.get(index-1) {
                                needed_hashes.insert(level, ProofNode::Left(sibling_node.hash().unwrap()));
                                next_hash = create_node_hash(sibling_node.hash().unwrap(), hash);
                            }
                        }

                    },
                    _ => continue
                };
            }
            level -= 1;
        }
        needed_hashes
    }

    fn get_element_index(&self, level: usize, hash: &String) -> Option<usize> {
        let row_hashes = self.nodes.get(&level).unwrap().iter()
            .map(|e| e.hash().unwrap()).collect::<Vec<&String>>();
        row_hashes.iter().position(|&s| s == hash)
    }
}

pub fn calculate_height(count: usize) -> usize {
    if count > 0 {
        let height = (count as f64).log2();
        if height - height.floor() > 0.0 {
            (height + 1.0) as usize
        } else {
            height as usize
        }
    } else {
        0
    }
}