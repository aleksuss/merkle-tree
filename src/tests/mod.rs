#![cfg(test)]

mod benchmarks;

use merkle_tree::{MerkleTree, calculate_height};
use hash_utils::*;

#[test]
fn test_empty_tree_hash() {
    let db: MerkleTree<u32> = MerkleTree::new();
    assert_eq!(&"5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9".to_string(),
               db.root_hash().unwrap_or(&"None".to_string()));
}

#[test]
fn test_height_and_len() {
    let root_hash = "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string();
    let mut db = MerkleTree::new();

    db.push("1");
    db.push("2");
    db.push("3");
    db.push("4");
    db.push("5");
    db.push("6");
    db.push("7");
    db.push("8");
    db.push("9");
    db.push("10");
    db.push("11");
    db.push("12");

    assert_eq!(12, db.len());
    assert_eq!(4, db.height());
    assert_eq!(&root_hash, db.root_hash().unwrap_or(&"None".to_string()));

    let good_proof = db.get_proof("6");
    assert_eq!(true, good_proof.validate(root_hash.as_ref()));

    let bad_proof = db.get_proof("1231231231");
    assert_eq!(false, bad_proof.validate(root_hash.as_ref()));
}

#[test]
fn test_get_element() {
    let mut db = MerkleTree::new();
    db.push(1);
    db.push(2);
    db.push(3);
    db.push(4);
    db.push(6664);

    assert_eq!(2, *db.get(1).unwrap());
    assert_eq!(6664, *db.get(4).unwrap());
}

#[test]
fn test_height_calc() {
    assert_eq!(0, calculate_height(0));
    assert_eq!(0, calculate_height(1));
    assert_eq!(1, calculate_height(2));
    assert_eq!(2, calculate_height(3));
    assert_eq!(2, calculate_height(4));
    assert_eq!(3, calculate_height(5));
    assert_eq!(3, calculate_height(8));
    assert_eq!(4, calculate_height(9));
}

#[test]
fn test_size() {
    let data = empty_hash();
    assert_eq!(64, data.len());
}

#[test]
fn test_hash() {
    assert_eq!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
               create_leaf_hash(&"hello".to_string()));
}

#[test]
fn test_combined_hash() {
    let hello_hash = create_leaf_hash(&"hello".to_string());
    let world_hash = create_leaf_hash(&"world".to_string());
    let node_hash = create_node_hash(&hello_hash, &world_hash);
    assert_eq!("15e178b71fae8849ee562c9cc0d7ea322fba6cd495411329d47234479167cc8b",
               node_hash);
}

use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
struct Person {
    age: usize,
    name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "age: {}, name: {}", self.age, self.name)
    }
}

#[test]
fn test_with_structs() {
    let mut db = MerkleTree::new();
    db.push(Person {
                age: 3,
                name: "Bob".to_string(),
            });
    db.push(Person {
                age: 4,
                name: "Bobb".to_string(),
            });
    db.push(Person {
                age: 5,
                name: "Bobbb".to_string(),
            });
    db.push(Person {
                age: 6,
                name: "Bobbbb".to_string(),
            });
    assert_eq!(4, db.len());

    let good_proof = db.get_proof(Person {
                                      age: 3,
                                      name: "Bob".to_string(),
                                  });
    assert!(good_proof.validate(db.root_hash().unwrap()));

    let bad_proof = db.get_proof(Person {
                                     age: 3,
                                     name: "Bobx".to_string(),
                                 });
    assert!(!bad_proof.validate(db.root_hash().unwrap()));

}

#[test]
fn test_append_element() {
    let mut db = MerkleTree::from_vec((0..1000).collect::<Vec<_>>());
    assert_eq!(1000, db.len());
    db.push(1000);
    assert_eq!(1001, db.len());
}

#[test]
fn test_remove_element() {
    let mut db = MerkleTree::from_vec((0..1000).collect::<Vec<_>>());
    assert_eq!(1000, db.len());
    db.remove(5);
    assert_eq!(999, db.len());
}

#[test]
fn test_get_values() {
    let db = MerkleTree::from_vec((0..5).collect::<Vec<_>>());
    assert_eq!(db.get_values(), Some(vec![0, 1, 2, 3, 4]));
}

#[test]
fn test_root_calculation() {
    let db = MerkleTree::from_vec(vec![1, 2, 3, 4]);
    let h1 = create_leaf_hash(&1);
    let h2 = create_leaf_hash(&2);
    let h3 = create_leaf_hash(&3);
    let h4 = create_leaf_hash(&4);
    let h12 = create_node_hash(&h1, &h2);
    let h34 = create_node_hash(&h3, &h4);
    let root = create_node_hash(&h12, &h34);
    assert_eq!(&root, db.root_hash().unwrap());

    let good_proof = db.get_proof(2);
    assert!(good_proof.validate(db.root_hash().unwrap()));
    let bad_proof = db.get_proof(663);
    assert!(!bad_proof.validate(db.root_hash().unwrap()));

}
