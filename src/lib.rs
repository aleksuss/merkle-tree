//! merkle-tree implement Merkle Tree in Rust.
//!
//! A disadvantage in current implementation is need to recalculate full tree when added
//! or removed element into/from tree. To increase performance we need to implement partial
//! tree modification when adding or removing element.

#![feature(test)]
#![feature(step_by)]

extern crate crypto;
extern crate test;

mod element;
mod hash_utils;
mod merkle_tree;

mod tests;

pub use self::merkle_tree::MerkleTree;
