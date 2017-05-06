#![cfg(test)]

use test::Bencher;
use merkle_tree::MerkleTree;
use hash_utils::*;
#[test]
fn test_height() {
    let mut db = MerkleTree::new();

    assert_eq!(&"5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9".to_string(),
    db.root_hash().unwrap_or(&"None".to_string()));

    db.append("1");
    db.append("2");
    db.append("3");
    db.append("4");
    db.append("5");
    db.append("6");
    db.append("7");
    db.append("8");
    db.append("9");
    db.append("10");
    db.append("11");
    db.append("12");

    assert_eq!(12, db.len());
    assert_eq!(4, db.height());
    assert_eq!(4, MerkleTree::calculate_height(db.len()));
    assert_eq!(&"8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string(),
    db.root_hash().unwrap_or(&"None".to_string()));

    assert_eq!(true, db.validate_element("6".to_string(),
                                         "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));
    assert_eq!(true, db.validate_element("11".to_string(),
                                         "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));
    assert_eq!(false, db.validate_element("14".to_string(),
                                          "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));
    assert_eq!(false, db.validate_element("14adsfasdfsad".to_string(),
                                          "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));
    assert_eq!(true, db.validate_element("1".to_string(),
                                         "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));
    assert_eq!(false, db.validate_element("1423232".to_string(),
                                          "8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string()));

}

#[test]
fn test_get_element() {
    let mut db = MerkleTree::new();
    db.append(1);
    db.append(2);
    db.append(3);
    db.append(4);
    db.append(6664);

    assert_eq!(2, *db.get(1).unwrap());
    assert_eq!(6664, *db.get(4).unwrap());
    assert!(db.validate_element(4, db.root_hash().unwrap().to_string()));

    //    db.print_nodes();
}

#[test]
fn test_height_calc() {
    assert_eq!(0, MerkleTree::calculate_height(0));
    assert_eq!(0, MerkleTree::calculate_height(1));
    assert_eq!(1, MerkleTree::calculate_height(2));
    assert_eq!(2, MerkleTree::calculate_height(3));
    assert_eq!(2, MerkleTree::calculate_height(4));
    assert_eq!(3, MerkleTree::calculate_height(5));
    assert_eq!(3, MerkleTree::calculate_height(8));
    assert_eq!(4, MerkleTree::calculate_height(9));
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
    assert_eq!("15e178b71fae8849ee562c9cc0d7ea322fba6cd495411329d47234479167cc8b", node_hash);
}

#[bench]
fn test_insert(b: &mut Bencher) {
    let mut db = MerkleTree::new();

    b.iter(|| {
        db.append("c");
    })
}

