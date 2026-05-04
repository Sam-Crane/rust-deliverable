// Benchmark comparing three Two Sets algorithms.
//
// Measures wall-clock time for each algorithm at several values of n
// and prints a comparison table.

use std::time::Instant;
use two_sets::{solve_greedy, solve_pairs, solve_recursive};

fn bench<F: Fn(u64) -> Option<(Vec<u64>, Vec<u64>)>>(f: F, n: u64, iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(n));
    }
    start.elapsed() / iterations
}

fn main() {
    // Use values where n % 4 == 0 so all algorithms produce a result.
    let values: &[u64] = &[1_000, 10_000, 100_000, 1_000_000];

    println!(
        "{:<12} {:>15} {:>15} {:>15}",
        "n", "Greedy", "Pair-Based", "Recursive"
    );
    println!("{}", "-".repeat(60));

    for &n in values {
        let iters = if n <= 10_000 { 1000 } else if n <= 100_000 { 100 } else { 10 };

        let t_greedy = bench(solve_greedy, n, iters);
        let t_pairs = bench(solve_pairs, n, iters);
        let t_recursive = bench(solve_recursive, n, iters);

        println!(
            "n={:<9} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, t_greedy, t_pairs, t_recursive
        );
    }
}
