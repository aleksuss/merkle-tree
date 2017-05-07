#![feature(test)]
#![feature(step_by)]
extern crate crypto;
extern crate test;

mod element;
mod hash_utils;
mod merkle_tree;

mod tests;
mod benchmarks;

pub use self::merkle_tree::MerkleTree;
