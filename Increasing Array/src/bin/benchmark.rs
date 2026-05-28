// Benchmark comparing three Increasing Array algorithms.
//
// Generates random arrays of varying sizes and measures wall-clock time
// for each algorithm, printing a comparison table.

use increasing_array::{solve_fold, solve_greedy, solve_prefix_max};
use std::time::Instant;

fn generate_array(n: usize, seed: u64) -> Vec<i64> {
    // Simple LCG pseudo-random generator for reproducible benchmarks.
    let mut rng = seed;
    (0..n)
        .map(|_| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((rng >> 33) % 1_000_000_000) as i64
        })
        .collect()
}

fn bench<F: Fn(&[i64]) -> i64>(f: F, data: &[i64], iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn main() {
    let sizes: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

    println!(
        "{:<12} {:>15} {:>15} {:>15}",
        "Array Size", "Greedy", "Fold", "Prefix Max"
    );
    println!("{}", "-".repeat(60));

    for &n in sizes {
        let data = generate_array(n, 42);
        let iters = if n <= 10_000 {
            1000
        } else if n <= 100_000 {
            100
        } else {
            10
        };

        let t_greedy = bench(solve_greedy, &data, iters);
        let t_fold = bench(solve_fold, &data, iters);
        let t_prefix = bench(solve_prefix_max, &data, iters);

        println!(
            "n={:<9} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, t_greedy, t_fold, t_prefix
        );
    }
}
