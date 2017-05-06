use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::string::ToString;
use std::fmt::Display;

pub fn empty_hash() -> String {
    create_leaf_hash(&0)
}

pub fn create_leaf_hash<T: Display>(input: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&input.to_string());
    let result = hasher.result_str();
    result
}

pub fn create_node_hash<T: AsRef<str>>(left: &T, right: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(left.as_ref());
    hasher.input_str(right.as_ref());
    let result = hasher.result_str();
    result
}

