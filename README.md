# merkle-tree

Merkle tree implementation in Rust

Example usage:

```javascript
extern crate merkle_tree;

use merkle_tree::MerkleTree;

let tree = MerkleTree::new();
tree.push(1);

assert_eq!(1, tree.len());
assert!(tree.validate_value(1, db.root_hash().unwrap()));

```

Check and validation:

```javascript
let db   = MerkleTree::from_vec(vec![1, 2, 3, 4]);
let h1   = create_leaf_hash(&1);
let h2   = create_leaf_hash(&2);
let h3   = create_leaf_hash(&3);
let h4   = create_leaf_hash(&4);
let h12  = create_node_hash(&h1, &h2);
let h34  = create_node_hash(&h3, &h4);
let root = create_node_hash(&h12, &h34);
assert_eq!(&root, db.root_hash().unwrap());
assert!(db.validate_element(2, db.root_hash().unwrap()));
```

A disadvantage in current implementation is need to recalculate full tree when added
or removed element into/from tree. To increase performance we need to implement partial
tree modification when adding or removing element.

### Benchmarks
This benchmark show time to validate present of element in tree with 10000 elements contains.

```
test benchmarks::benchmark_bad_validation  ... bench: 190,457 ns/iter (+/- 36,367)
test benchmarks::benchmark_good_validation ... bench:  98,222 ns/iter (+/- 14,063)
```
