use std::collections::VecDeque;

use element::Element;

pub struct MerkleTree {
    root: Element,
    height: usize,
    count: usize,
    storage: VecDeque<String>
}

impl MerkleTree {

    pub fn new() -> Self {
        MerkleTree {
            root: Element::empty(),
            height: 0,
            count: 0,
            storage: VecDeque::new()
        }
    }

    pub fn append(&mut self, value: &str) {
        self.storage.push_back(value.into());
        self.count = self.storage.len();
        self.calculate_tree();
    }

    pub fn get(&self, index: usize) -> Option<&String> {
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

    pub fn get_values(&self) -> Option<VecDeque<String>> {
        if self.storage.is_empty() {
            None
        } else {
            Some(self.storage.clone())
        }
    }

    pub fn calculate_tree(&mut self) {
        self.count = self.storage.len();
        self.height = 1;
        self.root = Element::empty();

        if !self.storage.is_empty() {
            let mut current_row = self.storage.iter()
                .map(|value| Element::create_leaf(&value))
                .collect::<VecDeque<Element>>();
            while current_row.len() > 1 {
                let mut next_row = VecDeque::new();
                while !current_row.is_empty() {
                    let node = if current_row.len() > 1 {
                        let left = current_row.pop_front().unwrap();
                        let right = current_row.pop_front().unwrap();
                        println!("left hash: {}\nright hash: {}", &left.hash().unwrap(), &right.hash().unwrap());
                        Element::create_node(&left, &right)
                    } else {
                        let left = current_row.pop_front().unwrap();
                        println!("left hash: {}\nright hash: {}", &left.hash().unwrap(), &left.hash().unwrap());
                        Element::create_node(&left, &left)
                    };

                    next_row.push_back(node);
                }
                self.height += 1;
                current_row = next_row;
            }
            self.root = current_row.pop_front().unwrap();
            println!("root hash: {}", self.root_hash().unwrap());
        }
    }
}

#[test]
fn test_height() {
    let mut db = MerkleTree::new();

    assert_eq!(&"5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9".to_string(),
    db.root_hash().unwrap());

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
    assert_eq!(5, db.height());
    assert_eq!(&"8fed6b1d66ea88efd0c1b7e752334a08128791e974dce6f4c14902fa0e33d5e1".to_string(),
        db.root_hash().unwrap());
}

#[test]
fn test_get_element() {
    let mut db = MerkleTree::new();
    db.append("1");
    db.append("2");
    db.append("3");
    db.append("4");
    db.append("xyz");

    assert_eq!(&"2".to_string(), db.get(1).unwrap());
    assert_eq!(&"xyz".to_string(), db.get(4).unwrap());
}
