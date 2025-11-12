#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod bwt;
mod fm;

use clap::Parser;
use std::path::{Path, PathBuf};

fn time<T>(name: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let x = f();
    let duration = start.elapsed();
    println!("{name:<10}: {duration:5.2?}");
    x
}

#[derive(clap::Parser)]
struct Args {
    reference: PathBuf,
    reads: PathBuf,
}

fn bwt(input: &Path, output: &Path) {
    let mut text = vec![];
    let mut reader = needletail::parse_fastx_file(input).unwrap();
    while let Some(record) = reader.next() {
        let record = record.unwrap();
        text.extend_from_slice(&record.seq());
    }
    // Map to 0123.
    for x in &mut text {
        *x = (*x >> 1) & 3;
    }

    // Caps-sa construction
    let bwt = time("simple-saca", || bwt::simple_saca(&text));

    // write output to path.bwt:
    std::fs::write(output, bwt).unwrap();

    // time("small-bwt", || drop(small_bwt(&text)));
    // let bwt = time("caps-sa", || caps_sa(&text));
}

fn map(bwt_path: &Path, reads_path: &Path) {
    eprintln!("Reading BWT from {}", bwt_path.display());
    let bwt = std::fs::read(bwt_path).unwrap();
    eprintln!("Building FM index & rank structure");
    let fm = time("FM build", || fm::FM::new(&bwt));

    let mut reader = needletail::parse_fastx_file(reads_path).unwrap();
    let mut total = 0;
    let mut mapped = 0;
    let mut total_matches = 0;
    let start = std::time::Instant::now();
    while let Some(record) = reader.next() {
        let record = record.unwrap();
        let seq = record.seq();
        total += 1;
        let packed = &seq.iter().map(|&x| (x >> 1) & 3).collect::<Vec<_>>();
        let matches = fm.query(packed);
        total_matches += matches;
        if matches > 0 {
            mapped += 1;
        }

        if total % 1024 == 0 {
            let duration = start.elapsed();
            eprint!(
                "Processed {:>8} reads ({:>8} mapped, {:>8} matches) in {:5.2?} ({:>6.2} reads/s)\r",
                total,
                mapped,
                total_matches,
                duration,
                total as f64 / duration.as_secs_f64()
            );
        }
    }
    eprintln!();
    println!("{} {}", "#reads", total);
    println!("{} {}", "#mapped", mapped);
    println!("{} {}", "#matches", total_matches);
}

fn main() {
    let args = Args::parse();
    let bwt_path = &args.reference.with_added_extension("bwt");
    if !bwt_path.exists() {
        eprintln!("Building BWT at {}", bwt_path.display());
        bwt(&args.reference, bwt_path);
    }

    map(bwt_path, &args.reads);
}
