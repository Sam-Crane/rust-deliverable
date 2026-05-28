// Benchmark comparing three Another Game algorithms.
//
// Three input shapes exercise different access patterns:
//   - All-even: forces full traversal for early-exit variants.
//   - Odd at end: worst case for early-exit, equal cost for OR-reduce.
//   - Odd at front: best case for early-exit.

use another_game::{solve_bitor, solve_iter, solve_scan};
use std::time::Instant;

fn all_even(n: usize) -> Vec<u64> {
    (0..n as u64).map(|i| (i + 1) * 2).collect()
}

fn odd_at_end(n: usize) -> Vec<u64> {
    let mut v = all_even(n);
    if let Some(last) = v.last_mut() {
        *last += 1;
    }
    v
}

fn odd_at_front(n: usize) -> Vec<u64> {
    let mut v = all_even(n);
    if let Some(first) = v.first_mut() {
        *first += 1;
    }
    v
}

fn bench<F: Fn(&[u64]) -> bool>(f: F, data: &[u64], iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn run_table(label: &str, builder: impl Fn(usize) -> Vec<u64>) {
    println!("\n=== {label} ===");
    println!(
        "{:<14} {:>15} {:>15} {:>15}",
        "Heap count", "Scan", "Iter::any", "Bitwise OR"
    );
    println!("{}", "-".repeat(62));

    for &n in &[1_000usize, 100_000, 1_000_000] {
        let v = builder(n);
        let iters = if n <= 1_000 {
            10_000
        } else if n <= 100_000 {
            1_000
        } else {
            100
        };

        let t_scan = bench(solve_scan, &v, iters);
        let t_iter = bench(solve_iter, &v, iters);
        let t_or = bench(solve_bitor, &v, iters);

        println!(
            "n={:<11} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, t_scan, t_iter, t_or
        );
    }
}

fn main() {
    run_table("All even (full traversal)", all_even);
    run_table("Odd at end (worst case for early-exit)", odd_at_end);
    run_table("Odd at front (best case for early-exit)", odd_at_front);
}
