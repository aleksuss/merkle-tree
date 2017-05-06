use std::collections::{BTreeMap, VecDeque};
use std::fmt::Display;

use element::Element;
use hash_utils::*;

enum ProofNode<'a> {
    Left(&'a String),
    Right(&'a String)
}

pub struct MerkleTree<T: ToString + Display> {
    root: Element<T>,
    height: usize,
    count: usize,
    storage: VecDeque<T>,
    nodes: BTreeMap<usize, VecDeque<Element<T>>>
}

impl<T: ToString + Display> MerkleTree<T> {

    pub fn new() -> Self {
        MerkleTree {
            root: Element::empty(),
            height: 0,
            count: 0,
            storage: VecDeque::new(),
            nodes: BTreeMap::new()
        }
    }

    pub fn append(&mut self, value: T) {
        self.storage.push_back(value);
        self.count = self.storage.len();
        self.calculate_tree();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.storage.get(index)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn root_hash(&self) -> Option<&String> {
        self.root.hash()
    }

//    pub fn get_values(&self) -> Option<VecDeque<&T>> {
//        if self.storage.is_empty() {
//            None
//        } else {
//            Some(self.storage.clone())
//        }
//    }

    pub fn calculate_tree(&mut self) {
        self.count = self.storage.len();
        self.height = Self::calculate_height(self.count);
        self.root = Element::empty();
        self.nodes.clear();
        let mut current_level = self.height;

        if !self.storage.is_empty() {
            let leaves = self.storage.iter()
                .map(|value| Element::create_leaf(&value))
                .collect::<VecDeque<Element<_>>>();
            self.nodes.insert(current_level, leaves);

            while current_level > 0 {
                let above_level = current_level - 1;
                let above_row = {
                    let mut row = VecDeque::new();
                    let current_row = self.nodes.get(&current_level).unwrap();

                    for i in (0..current_row.len()).step_by(2) {
                        let left = current_row.get(i).unwrap();
                        let right = current_row.get(i+1).unwrap_or(left);
                        let node = Element::create_node(left, right);
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

    pub fn validate_element(&self, value: T, root_hash: String) -> bool {
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

    fn get_needed_hashes(&self, value: &String) -> BTreeMap<usize, ProofNode> {
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

//    pub fn print_nodes(&self) {
//        for (i, nodes) in &self.nodes {
//            println!("level: {}", i);
//            for node in nodes {
//                println!("\thash node: {:?}", &node.hash().unwrap());
//            }
//
//        }
//    }

    fn get_element_index(&self, level: usize, hash: &String) -> Option<usize> {
        let row_hashes = self.nodes.get(&level).unwrap().iter()
            .map(|e| e.get_hash().unwrap()).collect::<Vec<&String>>();
        row_hashes.iter().position(|&s| s == hash)
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
}
