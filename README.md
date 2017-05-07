# merkle-tree


Merkle tree implementation in Rust

Example usage:

```
extern crate merkle_tree;

use merkle_tree::MerkleTree;

let tree = MerkleTree::new();
tree.append(1);

assert_eq!(1, tree.len());
assert!(tree.validate_value(1));

```

