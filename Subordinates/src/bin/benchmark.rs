// Benchmark comparing three Subordinates algorithms.
//
// Generates random trees of varying sizes and measures wall-clock time
// for each algorithm, printing a comparison table.

use std::time::Instant;
use subordinates::{solve_iterative, solve_recursive, solve_reverse_bfs};

/// Generate a random tree as a boss list of length n-1 using a simple LCG.
/// Each employee i (2..=n) chooses a boss uniformly from {1..i-1}, producing
/// a valid rooted tree.
fn generate_tree(n: usize, seed: u64) -> Vec<usize> {
    let mut rng = seed;
    (2..=n)
        .map(|i| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            (((rng >> 33) as usize) % (i - 1)) + 1
        })
        .collect()
}

fn bench<F: Fn(&[usize]) -> Vec<usize>>(f: F, data: &[usize], iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn main() {
    let sizes: &[usize] = &[1_000, 10_000, 100_000, 200_000];

    println!(
        "{:<12} {:>15} {:>15} {:>15}",
        "n", "Recursive", "Iterative DFS", "Reverse BFS"
    );
    println!("{}", "-".repeat(60));

    for &n in sizes {
        let tree = generate_tree(n, 42);
        let iters = if n <= 10_000 { 100 } else if n <= 100_000 { 20 } else { 10 };

        let t_rec = bench(solve_recursive, &tree, iters);
        let t_it = bench(solve_iterative, &tree, iters);
        let t_rb = bench(solve_reverse_bfs, &tree, iters);

        println!(
            "n={:<9} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, t_rec, t_it, t_rb
        );
    }
}
