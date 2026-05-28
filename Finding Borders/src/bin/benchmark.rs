// Benchmark comparing three Finding Borders algorithms.
//
// Generates test strings of varying sizes (random, all-same, repeating
// pattern) and measures wall-clock time for each algorithm.

use finding_borders::{solve_hashing, solve_kmp, solve_z};
use std::time::Instant;

/// Generate a random lowercase string with deterministic LCG.
fn random_string(n: usize, seed: u64) -> Vec<u8> {
    let mut rng = seed;
    (0..n)
        .map(|_| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            b'a' + ((rng >> 33) % 26) as u8
        })
        .collect()
}

/// Generate a string of all the same character — many borders.
fn all_same(n: usize) -> Vec<u8> {
    vec![b'a'; n]
}

/// Generate a string by repeating "abc" — periodic borders.
fn periodic(n: usize) -> Vec<u8> {
    let pattern = b"abc";
    (0..n).map(|i| pattern[i % pattern.len()]).collect()
}

fn bench<F: Fn(&[u8]) -> Vec<usize>>(f: F, data: &[u8], iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn run_table(label: &str, builder: impl Fn(usize) -> Vec<u8>) {
    println!("\n=== {label} ===");
    println!(
        "{:<14} {:>15} {:>15} {:>15}",
        "Length", "KMP", "Z-Algorithm", "Hashing"
    );
    println!("{}", "-".repeat(62));

    for &n in &[10_000usize, 100_000, 1_000_000] {
        let s = builder(n);
        let iters = if n <= 10_000 {
            200
        } else if n <= 100_000 {
            50
        } else {
            10
        };

        let t_kmp = bench(solve_kmp, &s, iters);
        let t_z = bench(solve_z, &s, iters);
        let t_h = bench(solve_hashing, &s, iters);

        println!("n={:<11} {:>15.3?} {:>15.3?} {:>15.3?}", n, t_kmp, t_z, t_h);
    }
}

fn main() {
    run_table("Random string", |n| random_string(n, 42));
    run_table("All same character", all_same);
    run_table("Periodic 'abc'", periodic);
}
