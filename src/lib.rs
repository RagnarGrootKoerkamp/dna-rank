#![allow(incomplete_features)]
#![feature(
    generic_const_exprs,
    portable_simd,
    coroutines,
    coroutine_trait,
    exact_div,
    associated_const_equality
)]

pub mod blocks;
pub mod count;
pub mod count4;
pub mod qwt;
pub mod rank9;
pub mod ranker;
pub mod super_block;

pub type Ranks = [u32; 4];
