use std::hint::black_box;

use dna_rank::{DnaRank, Rank};

fn bench_dna_rank<const STRIDE: usize>() {
    let n = 1_000_000;
    let q = 1_000_000;
    let seq = b"ACGT".repeat(n / 4);
    let rank = DnaRank::<STRIDE>::new(&seq);

    let queries = (0..q)
        .map(|_| rand::random_range(0..seq.len()))
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();
    for q in &queries {
        black_box(rank.ranks(*q));
    }
    let duration = start.elapsed();
    eprintln!(
        "DnaRank: {} ns/query",
        duration.as_nanos() as f32 / q as f32
    );
}

fn main() {
    bench_dna_rank::<1>();
    bench_dna_rank::<2>();
    bench_dna_rank::<4>();
    bench_dna_rank::<8>();
    bench_dna_rank::<16>();
    bench_dna_rank::<32>();
    bench_dna_rank::<64>();
    bench_dna_rank::<128>();
    bench_dna_rank::<256>();
    bench_dna_rank::<512>();
    bench_dna_rank::<1024>();
}
