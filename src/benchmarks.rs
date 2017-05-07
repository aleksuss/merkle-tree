#![cfg(test)]

use test::Bencher;
//use merkle_tree::MerkleTree;

#[bench]
fn benchmark_insert(b: &mut Bencher) {
//    let mut db = MerkleTree::new();
    let mut x = 0;

    b.iter(|| {
        x += 1;
    })
}