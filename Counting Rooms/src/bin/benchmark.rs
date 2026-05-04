// Benchmark comparing three Counting Rooms algorithms.
//
// Generates random grids of varying sizes (deterministic) and measures
// wall-clock time for each algorithm, printing a comparison table.

use std::time::Instant;
use counting_rooms::{solve_bfs, solve_dfs, solve_union_find};

/// Generate a random grid with ~70% floor / ~30% wall using a simple LCG.
fn generate_grid(n: usize, m: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = seed;
    (0..n)
        .map(|_| {
            (0..m)
                .map(|_| {
                    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                    if (rng >> 33) % 10 < 7 { b'.' } else { b'#' }
                })
                .collect()
        })
        .collect()
}

fn bench<F: Fn(&[Vec<u8>]) -> usize>(f: F, data: &[Vec<u8>], iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn main() {
    let sizes: &[(usize, usize)] = &[(100, 100), (300, 300), (500, 500), (1000, 1000)];

    println!(
        "{:<14} {:>15} {:>15} {:>15}",
        "Grid", "BFS", "Iterative DFS", "Union-Find"
    );
    println!("{}", "-".repeat(62));

    for &(n, m) in sizes {
        let grid = generate_grid(n, m, 42);
        let cells = n * m;
        let iters = if cells <= 10_000 { 100 } else if cells <= 100_000 { 30 } else { 5 };

        let t_bfs = bench(solve_bfs, &grid, iters);
        let t_dfs = bench(solve_dfs, &grid, iters);
        let t_uf = bench(solve_union_find, &grid, iters);

        println!(
            "{}x{:<10} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, m, t_bfs, t_dfs, t_uf
        );
    }
}
