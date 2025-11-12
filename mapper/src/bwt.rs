#![allow(unused)]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// BWT for context 100kbp.
pub fn simple_saca(text: &[u8]) -> Vec<u8> {
    let sa = simple_saca::suffix_array::SuffixArray::<5>::new_packed::<300>(text, 10, 6);

    sa.idxs()
        .par_iter()
        .map(|i| {
            let i = i.get_usize();
            if i == 0 {
                text[text.len() - 1]
            } else {
                text[i as usize - 1]
            }
        })
        .collect()
}

/// Needs external memory for human genome; 2x slower.
fn caps_sa(text: &[u8]) -> Vec<u8> {
    let mut sa = Vec::with_capacity(text.len());
    caps_sa_rs::build_sa_u8(text, &mut sa, true);

    sa.par_iter()
        .map(|&i| {
            if i == 0 {
                text[text.len() - 1]
            } else {
                text[i as usize - 1]
            }
        })
        .collect()
}

/// Text must have a \0 or $ at the end.
/// Much slower, and only single-threaded.
fn small_bwt(text: &[u8]) -> Vec<u8> {
    let mut bwt = Vec::with_capacity(text.len());
    small_bwt::verify_terminator(text).unwrap();
    small_bwt::BwtBuilder::new(text)
        .unwrap()
        .build(&mut bwt)
        .unwrap();
    bwt
}
