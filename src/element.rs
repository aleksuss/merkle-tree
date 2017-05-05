use hash_utils::*;

#[derive(Clone)]
pub enum Element {
    Node {
        left_node: Box<Element>,
        right_node: Box<Element>,
        hash: String
    },
    Leaf {
        data: String,
        hash: String
    },
    Empty {
        hash: String
    }
}

impl Element {
    pub fn empty() -> Self {
        Element::Empty {
            hash: empty_hash()
        }
    }

    pub fn hash(&self) -> Option<&String> {
        match *self {
            Element::Node  {ref hash, ..} |
            Element::Leaf  {ref hash, ..} |
            Element::Empty {ref hash    } => Some(hash)
        }
    }

    pub fn create_leaf(value: &String) -> Element {
        let leaf_hash = crate_leaf_hash(value);

        Element::Leaf {
            data: value.to_string(),
            hash: leaf_hash
        }
    }

    pub fn create_node(left: &Element, right: &Element) -> Element {
        let combined_hash = create_node_hash(left.hash().unwrap(), right.hash().unwrap());
        Element::Node {
            hash: combined_hash,
            left_node: Box::new(left.clone()),
            right_node: Box::new(right.clone())
        }

    }
}
