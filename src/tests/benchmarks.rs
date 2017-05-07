#![cfg(test)]
extern crate rand;

use test::Bencher;
use merkle_tree::MerkleTree;

#[bench]
fn benchmark_validation(b: &mut Bencher) {
    let data = (0..1000).collect::<Vec<_>>();
    let db = MerkleTree::from_vec(data);
    let root_hash = db.root_hash();

    b.iter(|| {
        db.validate_element(557, root_hash.unwrap().clone());
    })
}

#[bench]
fn benchmark_creation_from_vec(b: &mut Bencher) {
    b.iter(|| {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let _ = MerkleTree::from_vec(v);
    });
}
