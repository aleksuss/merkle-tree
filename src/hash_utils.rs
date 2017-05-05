use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::string::ToString;
use std::fmt::Display;

pub fn empty_hash() -> String {
    crate_leaf_hash(&0)
}

pub fn crate_leaf_hash<T: Display>(input: &T) -> String {
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

#[test]
fn test_size() {
    let data = empty_hash();
    assert_eq!(64, data.len());
}

#[test]
fn test_hash() {
    assert_eq!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
    crate_leaf_hash(&"hello".to_string()));
}

#[test]
fn test_combined_hash() {
    let hello_hash = crate_leaf_hash(&"hello".to_string());
    let world_hash = crate_leaf_hash(&"world".to_string());
    let node_hash = create_node_hash(&hello_hash, &world_hash);
    assert_eq!("15e178b71fae8849ee562c9cc0d7ea322fba6cd495411329d47234479167cc8b", node_hash);
}
