use std::fmt::Display;

use merkle_tree::ProofNode;
use hash_utils::{create_leaf_hash, create_node_hash};

#[derive(Debug)]
pub struct Proof<T: Display> {
    root_hash: String,
    value: T,
    path: Vec<ProofNode>,
}

impl<T> Proof<T>
    where T: Display
{
    pub fn new(root_hash: String, value: T, path: Vec<ProofNode>) -> Self {
        Proof {
            root_hash: root_hash,
            value: value,
            path: path,
        }
    }

    pub fn validate(&self, root_hash: &str) -> bool {
        let mut hash = create_leaf_hash(&self.value);

        for node in &self.path {
            hash = match node {
                &ProofNode::Left(ref proof_hash) => create_node_hash(proof_hash, &hash),
                &ProofNode::Right(ref proof_hash) => create_node_hash(&hash, proof_hash),
            };
        }

        hash == root_hash
    }
}
