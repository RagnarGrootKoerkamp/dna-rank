#![allow(incomplete_features)]
#![feature(
    generic_const_exprs,
    portable_simd,
    coroutines,
    coroutine_trait,
    exact_div,
    associated_const_equality
)]

use std::array::from_fn;

pub mod blocks;
pub mod count;
pub mod count4;
pub mod qwt;
pub mod rank9;
pub mod ranker;
pub mod super_block;

pub type Ranks = [u32; 4];

pub type QuadRank =
    ranker::Ranker<blocks::HexaBlockMid4, super_block::TrivialSB, count4::SimdCount10, false>;

fn add(a: Ranks, b: Ranks) -> Ranks {
    from_fn(|c| a[c] + b[c])
}

#[test]
fn test() {
    use ranker::RankerT;

    let text = b"AGCCTTAGCTGCGACAGAATGGATCAGAAAGCTTGAAAACTTAGAGCAAAAAATTGACTATTTTGACGAGTGTCTTCTTCCAGGCATTTTCACCATCGACGCGGATCCTCCAGACGAGTTGTTTCTTGATGAACTG";
    let ranker = QuadRank::new(text);

    assert_eq!(ranker.count(0), [0, 0, 0, 0]);

    eprintln!();
    let base = b"ACTG".map(|c| text.iter().filter(|x| **x == c).count() as u32);
    eprintln!("want: {:?}", base);
    let cnt = ranker.count(text.len());
    eprintln!("get {:?}", cnt);
    assert_eq!(cnt, base);
    // panic!();
}
